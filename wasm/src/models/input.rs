use crate::types::*;
use multimap::MultiMap;
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[derive(Debug, Deserialize)]
#[serde(from = "NetworkInput")]
pub struct Network {
    pub stations: HashMap<StationID, Station>,
    pub intervals: HashMap<IntervalID, Interval>,
    pub trains: HashMap<TrainID, Train>,
}

#[derive(Debug, Deserialize)]
struct NetworkInput {
    stations: HashMap<StationID, Station>,
    intervals: Vec<(IntervalID, Interval)>,
    trains: HashMap<TrainID, Train>,
}

impl From<NetworkInput> for Network {
    fn from(input: NetworkInput) -> Self {
        let mut intervals: HashMap<IntervalID, Interval> = input.intervals.into_iter().collect();
        for (id, train) in &input.trains {
            // use the window method
            for win in train.schedule.windows(2) {
                let beg = &win[0].0;
                let end = &win[1].0;
                // insert the train ID into the interval
                if let Some(interval) = intervals.get_mut(&(beg.clone(), end.clone())) {
                    interval.trains.insert(id.clone());
                }
                // otherwise do nothing
            }
        }
        let mut trains = input.trains;
        for (_, train) in trains.iter_mut() {
            for (i, (station_id, _)) in train.schedule.iter().enumerate() {
                train.indices.insert(station_id.clone(), i);
            }
        }
        Network {
            stations: input.stations,
            intervals,
            trains,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Station {
    pub tracks: u16,
}

#[derive(Debug, Deserialize)]
pub struct Interval {
    pub length: IntervalLength,
    #[serde(skip)]
    /// populated when the network is constructed
    pub trains: HashSet<TrainID>,
}

#[derive(Debug, Deserialize)]
pub struct Train {
    pub schedule: Vec<(StationID, ScheduleEntry)>,
    #[serde(skip)]
    pub indices: MultiMap<StationID, usize>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleEntry {
    pub arrival: Time,
    pub departure: Time,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub intervals_to_draw: Vec<(IntervalID, bool)>,
    pub position_scale_mode: ScaleMode,
    pub unit_length: GraphLength,
    pub time_scale_mode: ScaleMode,
}

impl NetworkConfig {
    pub fn intervals(&self) -> HashSet<IntervalID> {
        // collect intervals that are to be drawn
        // if the second element is true, reverse the interval
        self.intervals_to_draw
            .iter()
            .flat_map(|(id, reverse)| {
                if *reverse {
                    vec![id.clone()]
                } else {
                    vec![id.clone(), (id.1.clone(), id.0.clone())]
                }
            })
            .collect()
    }
    pub fn stations(&self) -> HashSet<StationID> {
        // collect all stations that are part of the intervals to be drawn
        self.intervals_to_draw
            .iter()
            .flat_map(|(id, _)| [id.0.clone(), id.1.clone()])
            .collect()
    }
}
