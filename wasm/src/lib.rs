use ciborium::{from_reader, into_writer};
use serde::Deserialize;
use typst_wasm_protocol::wasm_export;

mod models;
mod types;
mod utils;
use crate::types::{IntervalID, ScaleMode};
use models::graph::Network;
use models::input::{InExtras, InNetwork};
use models::output::OutDiagram;

#[derive(Deserialize)]
struct IntervalsToDraw(Vec<(IntervalID, Option<IntervalID>)>);

#[wasm_export]
pub fn process(in_network: &[u8], in_extras: &[u8]) -> Result<Vec<u8>, String> {
    let in_network: InNetwork =
        from_reader(in_network).map_err(|e| format!("Failed to parse input: {}", e))?;
    let in_extras: InExtras =
        from_reader(in_extras).map_err(|e| format!("Failed to parse intervals to draw: {}", e))?;
    let network =
        Network::new(&in_network).map_err(|e| format!("Failed to create network: {}", e))?;
    // serialize the output
    let out_diagram = OutDiagram::from_network(&network, &in_extras)
        .map_err(|e| format!("Failed to create output diagram: {}", e))?;
    let mut output = Vec::new();
    into_writer(&out_diagram, &mut output)
        .map_err(|e| format!("Failed to serialize output: {}", e))?;
    Ok(output)
}
