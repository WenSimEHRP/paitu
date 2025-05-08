use ciborium::{from_reader, into_writer, value::Value};
use sepax2d::{polygon, prelude::*};
use serde::{Deserialize, Serialize};
use std::vec;
use wasm_minimal_protocol::*;
initiate_protocol!();

#[inline(always)]
fn tausworthe(s: u32, a: u8, b: u8, c: u32, d: u8) -> u32 {
    let s1 = (s & c) << d;
    let s2 = ((s << a) ^ s) >> b;
    s1 ^ s2
}

#[derive(Serialize, Deserialize)]
struct Line {
    nodes: Vec<[f64; 2]>,
}

#[derive(Serialize, Deserialize)]
struct Station {
    id: String,
    position: f64,
    tracks: u64,
    labels: Vec<AABB>,
    lines: Vec<Line>,
    draw_height: f64,
}

#[derive(Serialize, Deserialize)]
struct Routing {
    trains: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Schedule {
    id: String,
    start: u64,
    end: u64,
}

#[derive(Serialize, Deserialize)]
struct Train {
    id: String,
    schedule: Vec<Schedule>,
    labels: Vec<Polygon>,
}

#[derive(Serialize, Deserialize)]
enum ScaleMode {
    Linear,
    Logarithmic,
    Uniform,
    Square,
    SquareRoot,
}

#[derive(Serialize, Deserialize)]
struct Diagram {
    stations: Vec<Station>,
    routings: Vec<Routing>,
    trains: Vec<Train>,
    station_draw_mode: ScaleMode,
}

impl Default for Station {
    fn default() -> Self {
        Station {
            id: String::new(),
            position: 0.0,
            tracks: 0,
            labels: Vec::new(),
            lines: Vec::new(),
            draw_height: 0.0,
        }
    }
}

fn process_stations(data: &Value, stations: &mut Vec<Station>) {
    // read the stuff in data
    // read some map settings and calculate the station positions
    // it is guaranteed that the data is a map, so don't check for that
    if let Value::Map(map) = data {
        for (name, stat) in map {
            if let Value::Map(map) = stat {
                let mut station = Station::default();
                for (k, v) in map {
                    match k.as_text().unwrap() {
                        "id" => {
                            station.id = v.as_text().unwrap().to_string();
                        }
                        "position" => {
                            if let Value::Float(f) = v {
                                station.position = *f;
                            } else if let Value::Integer(i) = v {
                                // convert to f64
                                station.position =
                                    <ciborium::value::Integer as TryInto<i64>>::try_into(*i)
                                        .unwrap() as f64;
                            }
                        }
                        "tracks" => {
                            station.tracks = v.as_integer().unwrap().try_into().unwrap();
                        }
                        _ => {}
                    }
                }
                stations.push(station);
            }
        }
    }
}

fn process_trains(data: &Value, trains: &mut Vec<Train>) {}

// not college board
fn solve_sat() {
    unimplemented!();
}

fn parse_data(data: &Value) -> Diagram {
    // Parse the CBOR data into the Diagram struct
    // check if data is a map
    // let mut collision: Vec<u8> = Vec::new();
    let mut d = Diagram {
        stations: Vec::new(),
        routings: Vec::new(),
        trains: Vec::new(),
        station_draw_mode: ScaleMode::Linear,
    };
    if let Value::Map(map) = data {
        for (k, v) in map {
            match k.as_text().unwrap() {
                "stations" => {
                    process_stations(v, &mut d.stations);
                }
                "trains" => {
                    process_trains(v, &mut d.trains);
                }
                _ => {}
            }
        }
        d
    } else {
        panic!("Expected a map");
    }
}

#[wasm_func]
pub fn return_cbor(data: &[u8]) -> Vec<u8> {
    let data: Value = from_reader(data).unwrap();
    // Modify the value as needed
    // For example, let's just add a new key-value pair
    let new_value = parse_data(&data);
    let mut output = Vec::new();
    match into_writer(&new_value, &mut output) {
        Ok(_) => output,
        Err(_) => panic!("Failed to write CBOR"),
    }
}
