use derive_more::{Add, AddAssign, Sub};
use serde::{Deserialize, Serialize};
use std::ops;

pub type StationID = String;
pub type TrainID = String;
pub type IntervalID = (StationID, StationID);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Add, Sub, Deserialize)]
pub struct Time(u32);

impl Time {
    pub fn new(seconds: u32) -> Self {
        Time(seconds)
    }
    pub fn seconds(&self) -> u32 {
        self.0
    }
    pub fn second(&self) -> u32 {
        self.0 % 60
    }
    pub fn minutes(&self) -> u32 {
        self.0 / 60
    }
    pub fn minute(&self) -> u32 {
        (self.0 / 60) % 60
    }
    pub fn hours(&self) -> u32 {
        self.0 / 3600
    }
    pub fn hour(&self) -> u32 {
        (self.0 / 3600) % 24
    }
    pub fn to_graph_length(&self, unit_length: GraphLength, scale_mode: ScaleMode) -> GraphLength {
        let hours = self.0 as f32 / 3600.0;
        unit_length * hours
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Add, Sub, Deserialize)]
pub struct IntervalLength(u32);

impl IntervalLength {
    pub fn kilometers(&self) -> f32 {
        self.0 as f32 / 1000.0
    }
    pub fn to_graph_length(&self, unit_length: GraphLength, scale_mode: ScaleMode) -> GraphLength {
        unit_length * self.kilometers()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add, Sub, Deserialize, Serialize, AddAssign)]
pub struct GraphLength(f32);

impl From<f32> for GraphLength {
    fn from(value: f32) -> Self {
        GraphLength(value)
    }
}

impl ops::Mul<GraphLength> for f32 {
    type Output = GraphLength;

    fn mul(self, rhs: GraphLength) -> Self::Output {
        GraphLength(self * rhs.0)
    }
}

impl ops::Mul<f32> for GraphLength {
    type Output = GraphLength;

    fn mul(self, rhs: f32) -> Self::Output {
        GraphLength(self.0 * rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ScaleMode {
    Linear,
    Logarithmic,
    Uniform,
}

#[derive(Debug, Serialize)]
pub struct Node(pub GraphLength, pub GraphLength);
