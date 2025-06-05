use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub type StationID = u32;
pub type IntervalID = (StationID, StationID);
pub type PolygonID = u32;
pub type DutyID = u32;
pub type TrainNumberID = u32;
pub type Track = u16;
/// interval length in meters, or seconds, depending on context
pub type IntervalLength = u32;
/// graph length in pts, the default unit of length in typst
pub type GraphLength = f32;
pub type Node = (GraphLength, GraphLength);
/// color degree in 0-360, for use with oklch
pub type ColorDegree = u16;

pub trait IntervalLengthExt {
    fn to_graph_length(self, scale: f32, mode: ScaleMode, unit_length: GraphLength) -> GraphLength;
}

impl IntervalLengthExt for IntervalLength {
    fn to_graph_length(self, scale: f32, mode: ScaleMode, unit_length: GraphLength) -> GraphLength {
        let length_km = self as f32 / 1000.0;
        let scale_factor = unit_length * scale; // 预计算

        let transformed = match mode {
            ScaleMode::Linear => length_km, // 最常用的放前面
            ScaleMode::Auto => length_km,
            ScaleMode::Uniform => 1.0,
            ScaleMode::Square => length_km * length_km,
            ScaleMode::SquareRoot => length_km.sqrt(),
            ScaleMode::Logarithmic => length_km.ln(),
        };
        transformed.max(1.0) * scale_factor
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Time(u32);

impl Time {
    pub fn from_hh_mm_ss_string(hh_mm_ss: String) -> Result<Self> {
        let parts: Vec<&str> = hh_mm_ss.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!(
                "Invalid time format: expected hh:mm:ss, got {}",
                hh_mm_ss
            ));
        }
        let h: u32 = parts[0].parse().with_context(|| {
            format!(
                "Failed to parse HOUR '{}' in time string '{}'",
                parts[0], hh_mm_ss
            )
        })?;
        let m: u32 = parts[1].parse().with_context(|| {
            format!(
                "Failed to parse MINUTE '{}' in time string '{}'",
                parts[1], hh_mm_ss
            )
        })?;
        let s: u32 = parts[2].parse().with_context(|| {
            format!(
                "Failed to parse SECOND '{}' in time string '{}'",
                parts[2], hh_mm_ss
            )
        })?;
        Ok(Time(h * 3600 + m * 60 + s))
    }
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
        self.0 / 3600 % 24
    }
}

#[derive(Copy, Clone, Deserialize)]
pub enum ScaleMode {
    Auto,
    Linear,
    Logarithmic,
    Square,
    SquareRoot,
    Uniform,
}
