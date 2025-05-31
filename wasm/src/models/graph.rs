use crate::{
    models::input::InNetwork,
    types::{IntervalID, IntervalLength, StationID, Time, Track, TrainNumberID},
};
use anyhow::{Context, Result};
use multimap::MultiMap;
use petgraph::graphmap::DiGraphMap;
use serde::Deserialize;
use std::collections::HashMap;

pub struct Interval {
    length: IntervalLength,
    trains: Vec<TrainNumberID>,
}

pub struct Station {
    tracks: Track,
    trains: Vec<TrainNumberID>,
}

pub struct Train {
    // a train can visit the same station multiple times
    entries: Vec<(StationID, Schedule)>,
    station_index: MultiMap<StationID, usize>, // index to the stations
}

#[derive(Deserialize, Clone)]
pub struct Schedule {
    arr: Time,
    dep: Time,
    track: Option<Track>,
}

pub struct Network {
    pub graph: DiGraphMap<StationID, Interval>,
    pub stations: HashMap<StationID, Station>,
    pub trains: HashMap<TrainNumberID, Train>,
}

impl Network {
    pub fn new(raw_input: &InNetwork) -> Result<Self> {
        let stations = &raw_input.stations;
        let intervals = &raw_input.intervals;
        let trains = &raw_input.trains;
        let mut station_map: HashMap<StationID, Station> = HashMap::with_capacity(stations.len());
        let mut graph: DiGraphMap<StationID, Interval> =
            DiGraphMap::with_capacity(stations.len(), intervals.len());
        let mut train_map: HashMap<TrainNumberID, Train> = HashMap::with_capacity(trains.len());
        // check stations
        anyhow::ensure!(
            !stations.is_empty(),
            "At least one station is required to create a network"
        );
        // check intervals
        anyhow::ensure!(
            !intervals.is_empty(),
            "At least one interval is required to create a network"
        );
        // we don't check trains here, as they can be empty
        // Initialize stations
        for (station_id, tracks) in stations {
            station_map.insert(
                *station_id,
                Station {
                    trains: Vec::new(),
                    tracks: *tracks,
                },
            );
        }
        // Initialize intervals
        for ((from, to), length) in intervals {
            graph.add_edge(
                *from,
                *to,
                Interval {
                    length: *length,
                    trains: Vec::new(),
                },
            );
        }
        // Initialize trains
        for (train_id, entries) in trains {
            let mut station_index = MultiMap::with_capacity(entries.len());
            for (i, (station_id, _)) in entries.iter().enumerate() {
                station_index.insert(*station_id, i);
            }
            train_map.insert(
                *train_id,
                Train {
                    entries: entries.clone(),
                    station_index,
                },
            );
        }
        let mut network = Self {
            graph,
            stations: station_map,
            trains: train_map,
        };
        network
            .finalize_trains()
            .context("Failed to finalize trains in the network")?;
        Ok(network)
    }
    fn finalize_trains(&mut self) -> Result<()> {
        let mut station_updates: HashMap<StationID, Vec<TrainNumberID>> =
            HashMap::with_capacity(self.stations.len());
        let mut interval_updates: HashMap<IntervalID, Vec<TrainNumberID>> =
            HashMap::with_capacity(self.graph.edge_count());

        // process train inputs
        for (train_id, train) in &self.trains {
            for window in train.entries.windows(2) {
                let [(curr_id, _), (next_id, _)] = window else {
                    continue;
                };

                station_updates.entry(*curr_id).or_default().push(*train_id);
                interval_updates
                    .entry((*curr_id, *next_id))
                    .or_default()
                    .push(*train_id);
            }

            if let Some((last_id, _)) = train.entries.last() {
                station_updates.entry(*last_id).or_default().push(*train_id);
            }
        }

        // process stations
        for (station_id, train_ids) in station_updates {
            if let Some(station) = self.stations.get_mut(&station_id) {
                station.trains.extend(train_ids); // batch process
            }
        }

        // process intervals
        for ((from, to), train_ids) in interval_updates {
            if let Some(interval) = self.graph.edge_weight_mut(from, to) {
                interval.trains.extend(train_ids); // batch process
            }
        }

        Ok(())
    }
    pub fn get_interval_length(&self, interval_id: IntervalID) -> Option<IntervalLength> {
        // given two station IDs, return the length of the interval
        let (from, to) = interval_id;
        self.graph
            .edge_weight(from, to)
            .map(|interval| interval.length)
    }
}
