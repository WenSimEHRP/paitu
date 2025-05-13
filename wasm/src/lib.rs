use ciborium::{from_reader, into_writer};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap};
use wasm_minimal_protocol::*;
initiate_protocol!();

// things to return

#[derive(Serialize, Deserialize, Default)]
struct Diagram {
    station_info: StationInfo,
    trains: HashMap<TrainID, Train>,
    polygons: HashMap<PolygonID, Polygon>,
    routings: HashMap<RoutingID, Routing>,
    collisions: Vec<Collision>,
    beg_x: f64,
    beg_y: f64,
    end_x: f64,
    end_y: f64,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Hash, Clone)]
struct StationID(String);

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Hash, Clone)]
struct TrainID(String);

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Hash, Clone)]
struct PolygonID(String);

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Hash, Clone)]
struct RoutingID(String);

#[derive(Serialize, Deserialize, Default)]
struct Nodes(Vec<[f64; 2]>);

#[derive(Serialize, Deserialize, Default)]
struct Collision(Vec<Nodes>);

#[derive(Serialize, Deserialize, Default)]
struct Polygon(Vec<Nodes>);

#[derive(Serialize, Deserialize, Default)]
struct Routing {
    // TODO
}

#[derive(Serialize, Deserialize, Default)]
struct Train {
    nodes: Vec<Nodes>,
    rand: u32,
}

#[derive(Serialize, Deserialize, Default)]
struct StationInfo {
    stations: IndexMap<StationID, Station>,
    scales: [f64; 24],
}

impl StationInfo {
    fn add_station(
        &mut self,
        id: StationID,
        station: &RawStation,
        scale: ScaleMode,
        position_axis_scale: f64,
        track_spacing_scale: f64,
    ) {
        let absolute_position: f64 = station.position;
        let relative_position: f64;
        let absolute_y: f64;
        let relative_y: f64;
        // check for the last station in the list
        if let Some(last_station) = self.stations.values().last() {
            relative_position = absolute_position - last_station.absolute_position;
            relative_y = (match scale {
                ScaleMode::Logarithmic => relative_position.ln(),
                ScaleMode::Square => relative_position.powf(2.0),
                ScaleMode::SquareRoot => relative_position.sqrt(),
                ScaleMode::Uniform => 0.0,
                ScaleMode::Linear => relative_position,
                ScaleMode::Auto => relative_position,
            })
            .max(1.0)
                * position_axis_scale;
            absolute_y = relative_y
                + (station.tracks.max(1) - 1) as f64 * track_spacing_scale
                + last_station.absolute_y;
        } else {
            relative_position = 0.0;
            relative_y = 0.0;
            absolute_y = (station.tracks.max(1) - 1) as f64 * track_spacing_scale;
        }
        self.stations.insert(
            id,
            Station {
                relative_position,
                absolute_position,
                relative_y,
                absolute_y,
                density: [[0; 6]; 24],
            },
        );
    }
    fn make_stations(
        stations: &HashMap<StationID, RawStation>,
        scale: ScaleMode,
        position_axis_scale: f64,
        track_spacing_scale: f64,
    ) -> Self {
        // sort the stations by their position
        let mut sorted_stations: Vec<_> = stations.iter().collect();
        sorted_stations.sort_by(|a, b| a.1.position.partial_cmp(&b.1.position).unwrap());
        // for each station, call the new function
        let mut station_info = StationInfo {
            stations: IndexMap::with_capacity(sorted_stations.len()),
            scales: [1.0; 24],
        };
        for (id, station) in sorted_stations {
            station_info.add_station(
                id.clone(),
                station,
                scale.clone(),
                position_axis_scale,
                track_spacing_scale,
            );
        }
        station_info
    }
    fn make_densities(&mut self, trains: &HashMap<TrainID, RawTrain>) {
        for (_, train) in trains {
            for i in 0..train.schedule.len() - 1 {
                let curr = &train.schedule[i];
                let next = &train.schedule[i + 1];
                let curr_id = curr.station.clone();
                let next_id = next.station.clone();
                // look up the station in the map
                // if it is not found, skip it
                if let Some(j) = self.stations.get_index_of(&curr_id) {
                    if let Some((key, _)) = self.stations.get_index(j + 1) {
                        if next_id != *key {
                            continue;
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
                // get the time difference
                for j in ((curr.time[1] / 600) as usize)..((next.time[0] / 600) as usize) {
                    // get the station
                    let station = self.stations.get_mut(&curr_id).unwrap();
                    // get the density
                    let density = &mut station.density[j / 6];
                    // increment the density
                    density[j % 6] += 1;
                }
            }
        }
    }

    fn make_scales(&mut self, time_axis_scale: f64, time_scale: ScaleMode) {
        // square root the density to get the scale
        // scales are in 24 steps, while the density is in 144 steps
        if time_scale != ScaleMode::Auto {
            self.scales = [time_axis_scale; 24];
            return;
        }
        for i in 0..24 {
            // sum the density for each 6 time slots
            let sum= self.stations.values().map(|station| {
                let density = &station.density[i];
                density.iter().sum::<u32>()
            }).sum::<u32>();
            self.scales[i] = (match sum {
                0 => 1.0,
                _ => (sum as f64).sqrt().round(),
            }).max(1.0) * time_axis_scale;
        }
    }

    fn new(
        stations: &HashMap<StationID, RawStation>,
        position_scale: ScaleMode,
        position_axis_scale: f64,
        track_spacing_scale: f64,
        time_axis_scale: f64,
        trains: &HashMap<TrainID, RawTrain>,
        time_scale: ScaleMode,
    ) -> Self {
        let mut station_info =
            Self::make_stations(stations, position_scale, position_axis_scale, track_spacing_scale);
        station_info.make_densities(trains);
        station_info.make_scales(time_axis_scale, time_scale);
        station_info
    }
}

#[derive(Serialize, Deserialize)]
struct Station {
    relative_position: f64,
    absolute_position: f64,
    relative_y: f64,
    absolute_y: f64,
    density: [[u32; 6]; 24],
}

impl Default for Station {
    fn default() -> Self {
        Station {
            relative_position: 0.0,
            absolute_position: 0.0,
            relative_y: 0.0,
            absolute_y: 0.0,
            density: [[0; 6]; 24],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
enum ScaleMode {
    Linear,
    Logarithmic,
    Square,
    SquareRoot,
    Uniform,
    Auto,
}

impl Default for ScaleMode {
    fn default() -> Self {
        ScaleMode::Linear
    }
}

#[derive(Serialize, Deserialize, Default)]
struct RawDiagram {
    trains: HashMap<TrainID, RawTrain>,
    stations: HashMap<StationID, RawStation>,
    polygons: HashMap<PolygonID, RawPolygon>,
    routings: HashMap<RoutingID, RawRouting>,
    position_scale: ScaleMode,
    time_scale: ScaleMode,
    beg_time: i32,
    end_time: i32,
    label_angle: f64,
    unit_length: f64,
    position_axis_scale: f64,
    track_spacing_scale: f64,
    time_axis_scale: f64,
}

#[derive(Serialize, Deserialize, Default)]
struct RawTrain {
    // label_size: [f64; 2], //TODO
    schedule: Vec<RawSchedule>,
}

#[derive(Serialize, Deserialize, Default)]
struct RawSchedule {
    station: StationID,
    time: [u32; 2],
    track: u64,
}

#[derive(Serialize, Deserialize, Default)]
struct RawStation {
    position: f64,
    tracks: u64,
}
#[derive(Serialize, Deserialize, Default)]
struct RawPolygon {
    // TODO
}
#[derive(Serialize, Deserialize, Default)]
struct RawRouting {
    // TODO
}

#[wasm_func]
pub fn process(data: &[u8]) -> Vec<u8> {
    let raw_data: RawDiagram = from_reader(data).unwrap();
    let wasm_response = process_diagram(&raw_data);
    let mut response: Vec<u8> = Vec::new();
    match into_writer(&wasm_response, &mut response) {
        Ok(_) => response,
        Err(e) => panic!("Failed to serialize response: {}", e),
    }
}

fn process_diagram(raw_data: &RawDiagram) -> Diagram {
    let mut diagram = Diagram::default();
    diagram.station_info = StationInfo::new(
        &raw_data.stations,
        raw_data.position_scale.clone(),
        raw_data.position_axis_scale,
        raw_data.track_spacing_scale,
        raw_data.time_axis_scale,
        &raw_data.trains,
        raw_data.time_scale.clone(),
    );
    diagram
}
