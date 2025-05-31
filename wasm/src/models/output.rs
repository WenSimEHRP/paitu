use crate::models::graph::Network;
use crate::models::input::InExtras;
use crate::types::{
    ColorDegree, GraphLength, IntervalID, IntervalLength, Node, ScaleMode, StationID,
    TrainNumberID, interval_length_to_graph_length,
};
use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct OutDiagram {
    // entries are for exporting, thus they don't need to be public,
    // nor do they need to be mutable
    /// grid intervals used by the grid function
    grid_intervals: Vec<GraphLength>,
    /// train lines
    trains: Vec<OutTrain>,
    /// collisions on the graph to be highlighted
    #[serde(skip_serializing_if = "Option::is_none")]
    graph_collisions: Option<OutGraphCollisions>,
    /// interval information, used for heatmap
    #[serde(skip_serializing_if = "Option::is_none")]
    intervals: Option<Vec<OutInterval>>,
    /// station information, used for occupancy map
    #[serde(skip_serializing_if = "Option::is_none")]
    stations: Option<Vec<OutStation>>,
}

#[derive(Serialize)]
struct OutGraphCollisions {
    x_min: GraphLength,
    x_max: GraphLength,
    y_min: GraphLength,
    y_max: GraphLength,
    /// collisions on the graph, each node is a collision point
    collisions: Vec<OutGraphCollision>,
}

impl OutGraphCollisions {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            x_min: f32::MAX,
            x_max: f32::MIN,
            y_min: f32::MAX,
            y_max: f32::MIN,
            collisions: Vec::with_capacity(capacity),
        }
    }
    fn add_collision(&mut self, collision: OutGraphCollision) {
        if collision.is_empty() {
            return;
        }
        // update the min/max values
        for node in &collision {
            self.x_min = self.x_min.min(node.0);
            self.x_max = self.x_max.max(node.0);
            self.y_min = self.y_min.min(node.1);
            self.y_max = self.y_max.max(node.1);
        }
        // add the collision
        self.collisions.push(collision);
    }
}

type OutGraphCollision = Vec<Node>;

#[derive(Serialize)]
struct OutTrain {
    /// train number, used to identify the train
    id: TrainNumberID,
    /// color degree generated from the train number
    color: ColorDegree,
    /// train line, each node is related to a station
    lines: Vec<Vec<Node>>,
}

#[derive(Serialize)]
struct OutInterval {}

#[derive(Serialize)]
struct OutStation {}

impl OutDiagram {
    pub fn from_network(network: &Network, input: &InExtras) -> Result<Self> {
        let hours_to_draw: u8 = {
            // a day only has 24 hours
            // say that the beg hour is 23, and the end hour is 7,
            // then draw the hours from 23 to 24 and from 0 to 7
            if input.beg_hour < input.end_hour {
                input.end_hour - input.beg_hour
            } else {
                24 - input.beg_hour + input.end_hour
            }
        };
        let mut grid_intervals: Vec<GraphLength> =
            Vec::with_capacity(input.intervals_to_draw.len());
        let mut trains: Vec<OutTrain> = Vec::with_capacity(network.trains.len());
        // Each train will have two labels; each station also has one label.
        //
        let mut graph_collisions: OutGraphCollisions = OutGraphCollisions::with_capacity(
            // each train has at least two labels
            if input.draw_labels {
                // +25% safety margin. Normal case is 2 labels per train.
                // worst case is infinite labels per train
                // which is not likely to happen
                (network.trains.len() as f32 * 2.5) as usize
            } else {
                0
            } + if input.draw_station_names {
                input.stations_to_draw.as_ref().unwrap().len() + 1
            } else {
                0
            } + if input.draw_hours {
                (hours_to_draw as usize + 1) * 2
            } else {
                0
            } + 2,
        );
        let mut intervals: Option<Vec<OutInterval>> = if input.draw_heatmap {
            Some(Vec::with_capacity(input.intervals_to_draw.len()))
        } else {
            None
        };
        let mut stations: Option<Vec<OutStation>> = if input.stations_to_draw.is_some() {
            Some(Vec::with_capacity(
                input.stations_to_draw.as_ref().unwrap().len(),
            ))
        } else {
            None
        };
        // populate grid intervals
        for int in &input.intervals_to_draw {
            let length = Self::calc_interval_length(
                network,
                *int,
                input.interval_scale_mode,
                input.scale, // scale is 1.0 for grid intervals
                input.unit_length,
            )
            .context(format!("Failed to calculate interval length: {:?}", int))?;
            grid_intervals.push(length);
        }
        Ok(Self {
            grid_intervals,
            trains,
            graph_collisions: if input.draw_collision {
                Some(graph_collisions)
            } else {
                None
            },
            intervals,
            stations,
        })
    }
    fn calc_interval_length(
        net: &Network,
        int: (IntervalID, bool),
        mode: ScaleMode,
        scale: f32,
        unit_length: GraphLength,
    ) -> Result<GraphLength> {
        let l0: Option<IntervalLength> = net.get_interval_length(int.0);
        // if true, l1 is the reverse of l0
        let l1: Option<IntervalLength> = if int.1 {
            net.get_interval_length((int.0.1, int.0.0))
        } else {
            None
        };
        match (l0, l1) {
            (Some(l0), Some(l1)) => {
                Ok(
                    ((interval_length_to_graph_length(l0, scale, mode, unit_length)
                        + interval_length_to_graph_length(l1, scale, mode, unit_length))
                        / 2.0)
                        .max(unit_length),
                )
            }
            (Some(l0), None) => {
                Ok(interval_length_to_graph_length(l0, scale, mode, unit_length).max(unit_length))
            }
            (None, Some(_)) => Err(anyhow::anyhow!("Interval not found: {:?}", int.0)),
            (None, None) => Err(anyhow::anyhow!(
                "Interval not found: {:?}, {:?}",
                int.0,
                int.1
            )),
        }
    }
}
