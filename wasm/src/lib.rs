use typst_wasm_protocol::wasm_export;
use ciborium::{from_reader, into_writer};

mod types;
mod models;
use models::input::{Network, NetworkConfig};
use models::output::Output;

#[wasm_export]
pub fn generate(network: &[u8], config: &[u8]) -> Vec<u8> {
    let mut output_vec = Vec::new();
    let network: Network = from_reader(network).unwrap();
    let config: NetworkConfig = from_reader(config).unwrap();
    let output: Output = Output::new(network, config);
    // serialize the output
    into_writer(&output, &mut output_vec).unwrap();
    output_vec
}
