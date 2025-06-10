use crate::models::input::{Interval, Network, NetworkConfig, ScheduleEntry, Train};
use crate::types::*;
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Output {
    trains: HashMap<TrainID, TrainOutput>,
    grid_intervals: Vec<GraphLength>,
}

#[derive(Debug, Serialize, Default)]
struct TrainOutput {
    nodes: Vec<(Node, Node)>,
}

impl Output {
    pub fn new(network: Network, config: NetworkConfig) -> Self {
        let mut trains = HashMap::new();
        let mut grid_intervals = Vec::new();
        let mut current_height = GraphLength::from(0.0);

        for ((beg_stat_id, end_stat_id), reverse) in &config.intervals_to_draw {
            // ✅ 分离：获取区间数据
            let interval = Self::get_interval(&network, &beg_stat_id, &end_stat_id)
                .expect("Interval not found");

            // ✅ 分离：计算绘制长度
            let draw_length = Self::calculate_draw_length(&interval, &config);

            // ✅ 分离：处理区间内的所有列车
            Self::process_interval_trains(
                &interval,
                &beg_stat_id,
                &end_stat_id,
                current_height,
                draw_length,
                &network,
                &config,
                &mut trains,
            );

            // ✅ 更新高度
            current_height += draw_length;
            grid_intervals.push(draw_length);
        }

        Output {
            trains,
            grid_intervals,
        }
    }

    // ✅ 获取区间数据
    fn get_interval<'a>(
        network: &'a Network,
        beg_stat_id: &StationID,
        end_stat_id: &StationID,
    ) -> Option<&'a Interval> {
        network
            .intervals
            .get(&(beg_stat_id.clone(), end_stat_id.clone()))
    }

    // ✅ 计算绘制长度
    fn calculate_draw_length(interval: &Interval, config: &NetworkConfig) -> GraphLength {
        interval
            .length
            .to_graph_length(config.unit_length, config.position_scale_mode)
    }

    // ✅ 处理区间内的所有列车
    fn process_interval_trains(
        interval: &Interval,
        beg_stat_id: &StationID,
        end_stat_id: &StationID,
        current_height: GraphLength,
        draw_length: GraphLength,
        network: &Network,
        config: &NetworkConfig,
        trains: &mut HashMap<TrainID, TrainOutput>,
    ) {
        for train_id in &interval.trains {
            // ✅ 确保列车在输出中存在
            trains
                .entry(train_id.clone())
                .or_insert_with(TrainOutput::default);

            let train = network.trains.get(train_id).expect("Train not found");

            // ✅ 分离：处理单个列车的路径
            Self::process_single_train(
                train,
                train_id,
                beg_stat_id,
                end_stat_id,
                current_height,
                draw_length,
                config,
                trains,
            );
        }
    }

    // ✅ 处理单个列车的路径生成
    fn process_single_train(
        train: &Train,
        train_id: &TrainID,
        beg_stat_id: &StationID,
        end_stat_id: &StationID,
        current_height: GraphLength,
        draw_length: GraphLength,
        config: &NetworkConfig,
        trains: &mut HashMap<TrainID, TrainOutput>,
    ) {
        let schedule_indices = train
            .indices
            .get_vec(beg_stat_id)
            .expect("Station not found in train schedule");

        for &schedule_idx in schedule_indices {
            // ✅ 分离：处理单个调度点
            Self::process_schedule_point(
                train,
                train_id,
                schedule_idx,
                end_stat_id,
                current_height,
                draw_length,
                config,
                trains,
            );
        }
    }

    // ✅ 处理单个调度点
    fn process_schedule_point(
        train: &Train,
        train_id: &TrainID,
        schedule_idx: usize,
        end_stat_id: &StationID,
        current_height: GraphLength,
        draw_length: GraphLength,
        config: &NetworkConfig,
        trains: &mut HashMap<TrainID, TrainOutput>,
    ) {
        let (_, curr_schedule_entry) = train
            .schedule
            .get(schedule_idx)
            .expect("Schedule index out of bounds");

        // ✅ 分离：创建当前站点的节点
        let station_nodes = Self::create_station_nodes(curr_schedule_entry, current_height, config);

        // 添加站点节点
        let train_output = trains.get_mut(train_id).unwrap();
        train_output.nodes.push(station_nodes);

        // ✅ 分离：处理到下一站的连接
        Self::process_next_station_connection(
            train,
            train_id,
            schedule_idx,
            end_stat_id,
            curr_schedule_entry,
            current_height,
            draw_length,
            config,
            trains,
        );
    }

    // ✅ 创建站点节点（到达/出发）
    fn create_station_nodes(
        schedule_entry: &ScheduleEntry,
        current_height: GraphLength,
        config: &NetworkConfig,
    ) -> (Node, Node) {
        let arrival_node = Node(
            schedule_entry
                .arrival
                .to_graph_length(config.unit_length, config.time_scale_mode),
            current_height,
        );

        let departure_node = Node(
            schedule_entry
                .departure
                .to_graph_length(config.unit_length, config.time_scale_mode),
            current_height,
        );

        (arrival_node, departure_node)
    }

    // ✅ 处理到下一站的连接
    fn process_next_station_connection(
        train: &Train,
        train_id: &TrainID,
        schedule_idx: usize,
        end_stat_id: &StationID,
        curr_schedule_entry: &ScheduleEntry,
        current_height: GraphLength,
        draw_length: GraphLength,
        config: &NetworkConfig,
        trains: &mut HashMap<TrainID, TrainOutput>,
    ) {
        if let Some((next_station_id, next_schedule_entry)) = train.schedule.get(schedule_idx + 1) {
            if next_station_id == end_stat_id {
                // ✅ 分离：创建连接节点
                let connection_nodes = Self::create_connection_nodes(
                    curr_schedule_entry,
                    next_schedule_entry,
                    current_height,
                    draw_length,
                    config,
                );

                let train_output = trains.get_mut(train_id).unwrap();
                train_output.nodes.push(connection_nodes);
            }
        }
    }

    // ✅ 创建连接节点（当前出发 -> 下一站到达）
    fn create_connection_nodes(
        curr_schedule_entry: &ScheduleEntry,
        next_schedule_entry: &ScheduleEntry,
        current_height: GraphLength,
        draw_length: GraphLength,
        config: &NetworkConfig,
    ) -> (Node, Node) {
        let departure_node = Node(
            curr_schedule_entry
                .departure
                .to_graph_length(config.unit_length, config.time_scale_mode),
            current_height,
        );

        let arrival_node = Node(
            next_schedule_entry
                .arrival
                .to_graph_length(config.unit_length, config.time_scale_mode),
            current_height + draw_length,
        );

        (departure_node, arrival_node)
    }
}
