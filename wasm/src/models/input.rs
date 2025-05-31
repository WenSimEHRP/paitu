use crate::types::{
    GraphLength, IntervalID, ScaleMode, StationID,
    TrainNumberID, Track, IntervalLength,
};
use serde::Deserialize;
use crate::models::graph::Schedule;

#[derive(Deserialize)]
pub struct InNetwork {
    pub stations: Vec<(StationID, Track)>,
    pub intervals: Vec<(IntervalID, IntervalLength)>,
    pub trains: Vec<(TrainNumberID, Vec<(StationID, Schedule)>)>,
}

#[derive(Deserialize)]
pub struct InExtras {
    pub intervals_to_draw: Vec<(IntervalID, bool)>,
    pub stations_to_draw: Option<Vec<StationID>>,
    pub draw_station_names: bool,
    pub draw_labels: bool,
    pub draw_hours: bool,
    pub draw_collision: bool,
    pub draw_heatmap: bool,
    pub interval_scale_mode: ScaleMode,
    pub beg_hour: u8,
    pub end_hour: u8,
    pub scale: f32,
    pub unit_length: GraphLength,
}
