use crate::types::*;
use anyhow::{Context, Result};
use multimap::MultiMap;
use petgraph::graphmap::DiGraphMap;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use std::collections::{HashMap, HashSet};

#[serde_as]
#[derive(Deserialize)]
pub struct NetworkInput {
    #[serde_as(as = "Vec<(_, _)>")]
    stations: HashMap<StationID, StationInput>,
    #[serde_as(as = "Vec<(_, _)>")]
    intervals: HashMap<IntervalID, IntervalInput>,
    #[serde_as(as = "Vec<(_, _)>")]
    train_numbers: HashMap<TrainNumberID, TrainNumberInput>,
}

#[derive(Deserialize)]
struct StationInput {
    tracks: Option<Track>,
}

#[derive(Deserialize)]
struct IntervalInput {
    length: IntervalLength,
}

struct TrainNumberInput {
    schedule: Vec<(IntervalID, ScheduleEntry)>,
    schedule_indexes: MultiMap<IntervalID, usize>,
}

impl<'de> Deserialize<'de> for TrainNumberInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TrainNumberInputRaw {
            schedule: Vec<(IntervalID, ScheduleEntry)>,
        }

        let raw = TrainNumberInputRaw::deserialize(deserializer)?;
        let mut schedule_indexes = MultiMap::new();

        for (index, (interval_id, _)) in raw.schedule.iter().enumerate() {
            schedule_indexes.insert(*interval_id, index);
        }

        Ok(TrainNumberInput {
            schedule: raw.schedule,
            schedule_indexes,
        })
    }
}

#[skip_serializing_none]
#[derive(Deserialize)]
struct ScheduleEntry {
    arr: Time,
    dep: Time,
    track: Option<Track>,
}

#[derive(Deserialize)]
pub struct NetworkConfig {
    intervals_to_draw: Vec<(IntervalID, bool)>,
    position_axis_scale: f32,
    position_axis_mode: ScaleMode,
    unit_length: GraphLength,
}

impl NetworkConfig {
    pub fn get_uniq_intervals(&self) -> HashSet<IntervalID> {
        let mut result = HashSet::with_capacity(self.intervals_to_draw.len() * 2);
        for &(id, draw_reverse) in &self.intervals_to_draw {
            result.insert(id);
            if draw_reverse {
                result.insert((id.1, id.0));
            }
        }
        result
    }

    pub fn get_uniq_stations(&self) -> HashSet<StationID> {
        let mut result = HashSet::with_capacity(self.intervals_to_draw.len() * 2);
        for &(id, _) in &self.intervals_to_draw {
            result.insert(id.0);
            result.insert(id.1);
        }
        result
    }
}

#[serde_as]
#[derive(Serialize)]
pub struct Network {
    #[serde(skip)]
    graph: DiGraphMap<StationID, IntervalLength>,
    #[serde_as(as = "Vec<(_, _)>")]
    stations: HashMap<StationID, Station>,
    #[serde_as(as = "Vec<(_, _)>")]
    intervals: HashMap<IntervalID, Interval>,
    #[serde_as(as = "Vec<(_, _)>")]
    trains: HashMap<TrainNumberID, Train>,
}

#[derive(Serialize)]
struct Station {
    tracks: Option<Track>,
    #[serde(skip)]
    label_size: Option<(f32, f32)>,
}

#[serde_as]
#[derive(Serialize)]
struct Interval {
    length: IntervalLength,
    draw_height: GraphLength,
    trains_per_10_minutes: [[usize; 6]; 24],
}

#[serde_as]
#[derive(Serialize)]
struct Train {
    #[serde(skip)]
    schedule: MultiMap<IntervalID, (Time, Time, Option<Track>)>,
    color: ColorDegree,
    nodes: Vec<Vec<Node>>,
}

struct NetworkIndex {
    station_neighbors: HashMap<StationID, HashSet<StationID>>,
    station_intervals: HashMap<StationID, HashSet<IntervalID>>,
    interval_trains: HashMap<IntervalID, HashSet<TrainNumberID>>,
    train_intervals: HashMap<TrainNumberID, HashSet<IntervalID>>,
}

impl NetworkIndex {
    fn build_from_input(input: &NetworkInput) -> Self {
        let mut station_neighbors = HashMap::with_capacity(input.stations.len());
        let mut station_intervals = HashMap::with_capacity(input.stations.len());
        let mut interval_trains = HashMap::with_capacity(input.intervals.len());
        let mut train_intervals = HashMap::with_capacity(input.train_numbers.len());

        // 单次遍历构建站点索引
        for &interval_id in input.intervals.keys() {
            station_intervals.entry(interval_id.0).or_insert_with(|| HashSet::with_capacity(4)).insert(interval_id);
            station_intervals.entry(interval_id.1).or_insert_with(|| HashSet::with_capacity(4)).insert(interval_id);

            station_neighbors.entry(interval_id.0).or_insert_with(|| HashSet::with_capacity(4)).insert(interval_id.1);
            station_neighbors.entry(interval_id.1).or_insert_with(|| HashSet::with_capacity(4)).insert(interval_id.0);
        }

        // 批量构建列车索引
        for (&train_id, train_input) in &input.train_numbers {
            let intervals: HashSet<IntervalID> = train_input.schedule_indexes.keys().cloned().collect();

            for &interval_id in &intervals {
                interval_trains.entry(interval_id).or_insert_with(|| HashSet::with_capacity(2)).insert(train_id);
            }

            train_intervals.insert(train_id, intervals);
        }

        NetworkIndex {
            station_neighbors,
            station_intervals,
            interval_trains,
            train_intervals,
        }
    }

    // O(1) 查找相邻站点
    fn get_neighbor_stations(&self, station_id: &StationID) -> Option<&HashSet<StationID>> {
        self.station_neighbors.get(station_id)
    }

    // O(1) 查找站点相关的所有区间
    fn get_station_intervals(&self, station_id: &StationID) -> Option<&HashSet<IntervalID>> {
        self.station_intervals.get(station_id)
    }

    // O(1) 查找区间上的所有列车
    fn get_interval_trains(&self, interval_id: &IntervalID) -> Option<&HashSet<TrainNumberID>> {
        self.interval_trains.get(interval_id)
    }
}

impl Network {
    pub fn from_network_input_and_config(
        input: NetworkInput,
        config: NetworkConfig,
    ) -> Result<Self> {
        // ✅ O(n) 预处理，构建索引
        let index = NetworkIndex::build_from_input(&input);
        let mut graph = DiGraphMap::new();

        // ✅ O(1) 获取配置中的目标站点和区间
        let target_stations = config.get_uniq_stations();
        let target_intervals = config.get_uniq_intervals();

        // ✅ O(k) 扩展相关站点，k 是目标站点数
        let mut all_needed_stations = HashSet::with_capacity(target_stations.len() * 4);
        all_needed_stations.extend(&target_stations);
        for &station_id in &target_stations {
            if let Some(neighbors) = index.get_neighbor_stations(&station_id) {
                all_needed_stations.extend(neighbors);
            }
        }

        // ✅ O(k) 获取所有相关区间
        let mut all_needed_intervals =
            HashSet::with_capacity(target_intervals.len() + all_needed_stations.len() * 2);
        all_needed_intervals.extend(&target_intervals);
        for &station_id in &all_needed_stations {
            if let Some(intervals) = index.get_station_intervals(&station_id) {
                all_needed_intervals.extend(intervals);
            }
        }

        // ✅ 严格错误处理：如果站点不存在则返回错误
        let stations: HashMap<StationID, Station> = all_needed_stations
            .iter()
            .map(|&station_id| {
                let station_input = input.stations.get(&station_id).ok_or_else(|| {
                    anyhow::anyhow!("Station with ID {:?} not found in input", station_id)
                })?;

                Ok((
                    station_id,
                    Station {
                        tracks: station_input.tracks,
                        label_size: None,
                    },
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        // ✅ 严格错误处理：如果区间不存在则返回错误
        let intervals: HashMap<IntervalID, Interval> = all_needed_intervals
            .iter()
            .map(|&interval_id| {
                let interval_input = input.intervals.get(&interval_id).ok_or_else(|| {
                    anyhow::anyhow!("Interval with ID {:?} not found in input", interval_id)
                })?;

                // 缓存计算结果
                let draw_height = interval_input.length.to_graph_length(
                    config.position_axis_scale,
                    config.position_axis_mode,
                    config.unit_length,
                );

                let interval = Interval {
                    length: interval_input.length,
                    draw_height,
                    trains_per_10_minutes: [[0; 6]; 24],
                };

                // 构建图
                graph.add_edge(interval_id.0, interval_id.1, interval.length);

                Ok((interval_id, interval))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        // ✅ O(k) 获取相关列车
        let mut relevant_trains = HashSet::new();
        for &interval_id in &all_needed_intervals {
            if let Some(trains) = index.get_interval_trains(&interval_id) {
                relevant_trains.extend(trains);
            }
        }

        // ✅ 严格错误处理：如果列车不存在则返回错误
        let trains: HashMap<TrainNumberID, Train> = relevant_trains
            .iter()
            .map(|&train_id| {
                let train_input = input.train_numbers.get(&train_id).ok_or_else(|| {
                    anyhow::anyhow!("Train with ID {:?} not found in input", train_id)
                })?;

                let filtered_schedule =
                    Self::filter_train_schedule_fast(train_input, &all_needed_intervals);

                if filtered_schedule.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Train {:?} has no schedule entries for the required intervals",
                        train_id
                    ));
                }

                Ok((
                    train_id,
                    Train {
                        schedule: filtered_schedule,
                        color: Self::generate_color(train_id),
                        nodes: Vec::new(),
                    },
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Network {
            graph,
            stations,
            intervals,
            trains,
        })
    }

    // ✅ 其他方法保持不变
    fn filter_train_schedule_fast(
        train_input: &TrainNumberInput,
        needed_intervals: &HashSet<IntervalID>,
    ) -> MultiMap<IntervalID, (Time, Time, Option<Track>)> {
        let mut filtered_schedule = MultiMap::new();

        for &interval_id in needed_intervals {
            if let Some(indexes) = train_input.schedule_indexes.get_vec(&interval_id) {
                for &index in indexes {
                    if let Some((_, entry)) = train_input.schedule.get(index) {
                        filtered_schedule.insert(interval_id, (entry.arr, entry.dep, entry.track));
                    }
                }
            }
        }

        filtered_schedule
    }

    fn generate_color(train_id: TrainNumberID) -> ColorDegree {
        ((train_id.wrapping_mul(2654435761)) % 360) as ColorDegree
    }
}
