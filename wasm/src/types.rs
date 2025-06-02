use anyhow::{Context, Result};
use bimap::BiMap;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::ops::Sub;

pub type StationID = u32;
pub type IntervalID = (StationID, StationID);
pub type PolygonID = u32;
pub type DutyID = u32; // duty corresponds to the actual train
pub type TrainNumberID = u32;
pub type Track = u16;
pub type IntervalLength = u32; // in meters
pub type GraphLength = f32; // in pt, used for graph coordinates
pub type Node = (GraphLength, GraphLength);
pub type ColorDegree = u16; // from 0 to 360, to be used with OkLch color space

#[inline]
pub fn interval_length_to_graph_length(
    length: IntervalLength,
    scale: f32,
    mode: ScaleMode,
    unit_length: GraphLength
) -> GraphLength {
    let length_km = length as f32 / 1000.0;
    let scale_factor = unit_length * scale; // 预计算

    let transformed = match mode {
        ScaleMode::Linear => length_km,  // 最常用的放前面
        ScaleMode::Auto => length_km,
        ScaleMode::Uniform => 1.0,
        ScaleMode::Square => length_km * length_km,
        ScaleMode::SquareRoot => length_km.sqrt(),
        ScaleMode::Logarithmic => length_km.ln(),
    };

    transformed.max(1.0) * scale_factor
}

pub struct StationRegistry {
    name_id_map: BiMap<String, StationID>,
    next_id: StationID,
}

impl StationRegistry {
    pub fn new() -> Self {
        Self {
            name_id_map: BiMap::new(),
            next_id: 1,
        }
    }

    pub fn get_or_create_id(&mut self, name: &str) -> StationID {
        if let Some(&id) = self.name_id_map.get_by_left(name) {
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.name_id_map.insert(name.to_string(), id);
            id
        }
    }

    pub fn get_name(&self, id: StationID) -> Option<&String> {
        self.name_id_map.get_by_right(&id)
    }

    pub fn get_id(&self, name: &str) -> Option<StationID> {
        self.name_id_map.get_by_left(name).copied()
    }

    pub fn len(&self) -> usize {
        self.name_id_map.len()
    }

    pub fn contains_name(&self, name: &str) -> bool {
        self.name_id_map.contains_left(name)
    }

    pub fn contains_id(&self, id: StationID) -> bool {
        self.name_id_map.contains_right(&id)
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
        let h: u32 = parts[0].parse()?;
        let m: u32 = parts[1].parse()?;
        let s: u32 = parts[2].parse()?;
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
    pub fn minute(&self) -> u32 {
        (self.0 / 60) % 60
    }
    pub fn minutes(&self) -> u32 {
        self.0 / 60
    }
    pub fn hour(&self) -> u32 {
        self.0 / 3600 % 24
    }
    pub fn hours(&self) -> u32 {
        self.0 / 3600
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
