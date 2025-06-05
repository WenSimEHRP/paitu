use ciborium::{from_reader, into_writer};
use serde::Deserialize;
use typst_wasm_protocol::wasm_export;

use crate::models::input::{NetworkConfig, NetworkInput, Network};

mod models;
mod types;
mod utils;

#[wasm_export]
pub fn process(in_network: &[u8], in_extras: &[u8]) -> Result<Vec<u8>, String> {
    let network_input: NetworkInput = ciborium::from_reader(in_network)
        .map_err(|e| format!("Failed to parse network input: {}", e))?;
    let network_config: NetworkConfig = ciborium::from_reader(in_extras)
        .map_err(|e| format!("Failed to parse network config: {}", e))?;
    // Process the network input and config
    let network = Network::from_network_input_and_config(network_input, network_config)
        .map_err(|e| format!("Failed to create network: {}", e))?;
    // Serialize the output
    let mut output_bytes = Vec::new();
    into_writer("123123", &mut output_bytes)
        .map_err(|e| format!("Failed to serialize output: {}", e))?;
    Ok(output_bytes)
}

#[wasm_export]
pub fn make_bimap(input: &[u8]) -> Result<Vec<u8>, String> {
    // input is a vector of strings
    let strings: Vec<String> = ciborium::from_reader(input)
        .map_err(|e| format!("Failed to parse input: {}", e))?;
    // don't actually use a bimap. In this case, return a hashmap and a vector
    let mut map = std::collections::HashMap::new();
    let mut vec = Vec::new();
    for (i, s) in strings.iter().enumerate() {
        map.insert(s.clone(), i);
        vec.push(s.clone());
    }
    // serialize the output
    let output = (map, vec);
    let mut output_bytes = Vec::new();
    into_writer(&output, &mut output_bytes)
        .map_err(|e| format!("Failed to serialize output: {}", e))?;
    Ok(output_bytes)
}
