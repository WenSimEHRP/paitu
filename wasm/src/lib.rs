use ciborium::{from_reader, into_writer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec;
use wasm_minimal_protocol::*;
initiate_protocol!();

// some constants
const STATION_LABEL_PADDING: f64 = 3.0;

#[inline(always)]
fn tausworthe(s: u32, a: u8, b: u8, c: u32, d: u8) -> u32 {
    let s1 = (s & c) << d;
    let s2 = ((s << a) ^ s) >> b;
    s1 ^ s2
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Label {
    coor: [f64; 2],
    index: usize,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Line {
    nodes: Vec<[f64; 2]>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Diagram {
    stations: HashMap<String, Station>,
    trains: HashMap<String, Train>,
    routings: HashMap<String, Routing>,
    collisions: Vec<Collision>,
    polygons: Vec<Polygon>,
    beg_x: f64,
    end_x: f64,
    beg_y: f64,
    end_y: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Station {
    lines: Vec<Line>,
    labels: Vec<Label>,
    draw_height: f64,
    rel_height: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Train {
    lines: Vec<Line>,
    labels: Vec<Label>,
    rand: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Routing {
    lines: Vec<Line>,
    labels: Vec<Label>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Collision {
    nodes: Vec<[f64; 2]>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Polygon {
    nodes: Vec<[f64; 2]>,
}

#[derive(Serialize, Deserialize, Debug)]
enum ScaleMode {
    Linear,
    Logarithmic,
    SquareRoot,
    Square,
    Uniform,
}

impl Default for ScaleMode {
    fn default() -> Self {
        ScaleMode::Linear
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawDiagram {
    stations: HashMap<String, RawStation>,
    trains: HashMap<String, RawTrain>,
    routings: Vec<RawRouting>,
    polygons: Vec<RawPolygon>,
    angle: f64,
    station_scale_mode: ScaleMode,
    label_angle: f64, // in degrees
    unit_length: f64, // in points
    position_axis_scale: f64,
    track_spacing_scale: f64,
    time_axis_scale: f64,
    beg_time: i32,
    end_time: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawStation {
    label: [f64; 2],
    position: f64,
    tracks: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawTrain {
    label: [f64; 2],
    schedule: Vec<RawStationSchedule>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawStationSchedule {
    time: [i32; 2],
    id: String, // station id
    track: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawTrainSchedule {
    id: String, // station id
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawRouting {
    id: String,
    schedule: Vec<RawTrainSchedule>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawPolygon {
    nodes: Vec<[f64; 2]>,
}

fn parse_stations(data: &RawDiagram, dia: &mut Diagram) {
    // sort the stations by their position first
    let mut stations_vec: Vec<(&String, &RawStation)> = data.stations.iter().collect();
    stations_vec.sort_by(|a, b| {
        a.1.position
            .partial_cmp(&b.1.position)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut draw_height = 0.0;
    let mut rel_height = 0.0;
    let mut prev_pos = stations_vec[0].1.position;
    let mut l;
    for (i, (id, st)) in stations_vec.into_iter().enumerate() {
        assert!(st.tracks > 0);
        let mut lines: Vec<Line> = Vec::new();
        l = st.position - prev_pos;
        match data.station_scale_mode {
            ScaleMode::Logarithmic => {
                if l >= 2.0 {
                    l = l.log2();
                } else {
                    l = 0.0;
                }
            }
            ScaleMode::SquareRoot => {
                l = l.sqrt();
            }
            ScaleMode::Square => {
                l = l.powi(2);
            }
            ScaleMode::Uniform => {
                if i == 0 {
                    l = 0.0;
                } else {
                    l = 1.0;
                }
            }
            _ => {
                // if ScaleMode::Linear or any other
                l = st.position - prev_pos;
            }
        }
        l *= data.unit_length * data.position_axis_scale;
        rel_height = l + (st.tracks - 1) as f64 * data.track_spacing_scale;
        draw_height += rel_height;
        prev_pos = st.position;
        dia.collisions.push(Collision {
            nodes: vec![
                [
                    dia.beg_x - st.label[0] - STATION_LABEL_PADDING,
                    draw_height - st.label[1] * 0.5,
                ],
                [
                    dia.beg_x - STATION_LABEL_PADDING,
                    draw_height - st.label[1] * 0.5,
                ],
                [
                    dia.beg_x - STATION_LABEL_PADDING,
                    draw_height + st.label[1] * 0.5,
                ],
                [
                    dia.beg_x - st.label[0] - STATION_LABEL_PADDING,
                    draw_height + st.label[1] * 0.5,
                ],
            ],
        });
        for i in 0..st.tracks {
            let height = draw_height - ((i as f64) * data.track_spacing_scale);
            lines.push(Line {
                nodes: vec![[dia.beg_x, height], [dia.end_x, height]],
            });
        }
        dia.stations.insert(
            id.clone(),
            Station {
                lines: lines,
                labels: Vec::new(),
                draw_height: draw_height,
                rel_height: rel_height,
            },
        );
    }
    dia.end_y = draw_height;
}

#[inline(always)]
fn solve_y_point(p1: [f64; 2], p2: [f64; 2], x: f64) -> [f64; 2] {
    if (p2[0] - p1[0]).abs() < 1e-10 {
        return [x, p1[1]];
    }
    let m = (p2[1] - p1[1]) / (p2[0] - p1[0]);
    let b = p1[1] - m * p1[0];
    [x, m * x + b]
}

fn parse_trains(data: &RawDiagram, dia: &mut Diagram) {
    for (id, train) in data.trains.iter() {
        let mut lines: Vec<Line> = Vec::new();
        let mut line = Line::default();
        for (i, stat) in train.schedule.iter().enumerate() {
            if !dia.stations.contains_key(&stat.id) {
                continue;
            }
            get_curr_station_nodes(data, stat, dia, &mut lines, &mut line);
            if i >= train.schedule.len() - 1 {
                break;
            }
            get_next_station_nodes(
                data,
                stat,
                &train.schedule[i + 1],
                dia,
                &mut lines,
                &mut line,
            );
        }
        lines.push(line);
        // filter out lines with less than 2 nodes
        lines.retain(|l| l.nodes.len() > 1);
        dia.trains.insert(
            id.clone(),
            Train {
                lines: lines,
                labels: Vec::new(),
                // generate a random number for the train
                rand: {
                    let mut seed = 0u32;
                    for (i, c) in id.bytes().enumerate() {
                        seed = seed.wrapping_add((c as u32) << (i % 4 * 8));
                    }
                    let s1 = tausworthe(seed, 13, 19, 4294967294, 12);
                    let s2 = tausworthe(s1, 2, 25, 4294967288, 4);
                    let s3 = tausworthe(s2, 3, 11, 4294967280, 17);

                    // 组合多个 Tausworthe 输出以提高随机性
                    s1 ^ s2 ^ s3
                },
            },
        );
    }
}

fn refresh_lines(lines: &mut Vec<Line>, line: &mut Line) {
    lines.push(line.clone());
    line.nodes.clear();
}

fn get_curr_station_nodes(
    data: &RawDiagram,
    stat: &RawStationSchedule,
    dia: &Diagram,
    lines: &mut Vec<Line>,
    line: &mut Line,
) {
    let curr_y = dia.stations[&stat.id].draw_height;
    let curr_track_y = curr_y - (stat.track as f64) * data.track_spacing_scale;
    let arr_time = stat.time[0] % 86400;
    let dep_time = stat.time[1] % 86400;
    let arr_x = arr_time as f64 / 3600.0 * data.unit_length * data.time_axis_scale;
    let dep_x = dep_time as f64 / 3600.0 * data.unit_length * data.time_axis_scale;
    let y = curr_track_y;
    if arr_time < data.beg_time {
        if dep_time < data.beg_time {
            // do nothing
        } else if dep_time < data.end_time {
            refresh_lines(lines, line);
            line.nodes.push([dia.beg_x, y]);
            line.nodes.push([dep_x, y]);
        } else {
            refresh_lines(lines, line);
            line.nodes.push([dia.beg_x, y]);
            line.nodes.push([dia.end_x, y]);
            refresh_lines(lines, line);
        }
    } else if arr_time < data.end_time {
        if dep_time < data.beg_time {
            line.nodes.push([arr_x, y]);
            line.nodes.push([dia.end_x, y]);
            refresh_lines(lines, line);
        } else if dep_time < arr_time {
            line.nodes.push([arr_x, y]);
            line.nodes.push([dia.end_x, y]);
            refresh_lines(lines, line);
            line.nodes.push([dia.beg_x, y]);
            line.nodes.push([dep_x, y]);
        } else if dep_time == arr_time {
            line.nodes.push([arr_x, y]);
        } else if dep_time < data.end_time {
            line.nodes.push([arr_x, y]);
            line.nodes.push([dep_x, y]);
        } else {
            line.nodes.push([arr_x, y]);
            line.nodes.push([dia.end_x, y]);
            refresh_lines(lines, line);
        }
    } else {
        if dep_time < data.beg_time {
            // do nothing
        } else if dep_time < data.end_time {
            refresh_lines(lines, line);
            line.nodes.push([dia.beg_x, y]);
            line.nodes.push([dep_x, y]);
        } else if dep_time < arr_time {
            refresh_lines(lines, line);
            line.nodes.push([dia.beg_x, y]);
            line.nodes.push([dia.end_x, y]);
            refresh_lines(lines, line);
        } else {
            // do nothing
        }
    }
}

fn get_next_station_nodes(
    data: &RawDiagram,
    cstat: &RawStationSchedule,
    nstat: &RawStationSchedule,
    dia: &Diagram,
    lines: &mut Vec<Line>,
    line: &mut Line,
) {
    if nstat.id == cstat.id || !dia.stations.contains_key(&nstat.id) {
        return;
    }
    let dep_time = cstat.time[1] % 86400;
    let nex_time = nstat.time[0] % 86400;
    let pdep_time = dep_time - 86400;
    let nnex_time = nex_time + 86400;
    let dep_x = dep_time as f64 / 3600.0 * data.unit_length * data.time_axis_scale;
    let nex_x = nex_time as f64 / 3600.0 * data.unit_length * data.time_axis_scale;
    let pdep_x = pdep_time as f64 / 3600.0 * data.unit_length * data.time_axis_scale;
    let nnex_x = nnex_time as f64 / 3600.0 * data.unit_length * data.time_axis_scale;
    // temp
    let mut cy = dia.stations[&cstat.id].draw_height;
    let mut ny = dia.stations[&nstat.id].draw_height
        - (data.stations[&nstat.id].tracks - 1) as f64 * data.track_spacing_scale;
    if ny < cy {
        cy = dia.stations[&cstat.id].draw_height
            - (data.stations[&cstat.id].tracks - 1) as f64 * data.track_spacing_scale;
        ny = dia.stations[&nstat.id].draw_height;
    }
    let track_cond = true // track_scale != 0
        && (
          if cy < ny {
            cstat.track > 0
          } else {
            cstat.track + 1 < data.stations[&cstat.id].tracks
          }
        );
    let next_track_cond = true // track_scale != 0
        && (
          if cy < ny {
            nstat.track + 1 < data.stations[&nstat.id].tracks
          } else {
            nstat.track > 0
          }
        );
    if dep_time < data.beg_time {
        if nex_time < data.beg_time {
            // do nothing
        } else if nex_time < data.end_time {
            refresh_lines(lines, line);
            line.nodes
                .push(solve_y_point([dep_x, cy], [nex_x, ny], dia.beg_x));
            line.nodes.push([nex_x, ny]);
        } else {
            refresh_lines(lines, line);
            line.nodes
                .push(solve_y_point([dep_x, cy], [nex_x, ny], dia.beg_x));
            line.nodes
                .push(solve_y_point([dep_x, cy], [nex_x, ny], dia.end_x));
            refresh_lines(lines, line);
        }
    } else if dep_time < data.end_time {
        if nex_time < data.beg_time {
            if track_cond {
                line.nodes.push([dep_x, cy])
            }
            line.nodes
                .push(solve_y_point([dep_x, cy], [nnex_x, ny], dia.end_x));
            refresh_lines(lines, line);
        } else if nex_time < dep_time {
            if track_cond {
                line.nodes.push([dep_x, cy])
            }
            line.nodes
                .push(solve_y_point([dep_x, cy], [nnex_x, ny], dia.end_x));
            refresh_lines(lines, line);
            line.nodes
                .push(solve_y_point([pdep_x, cy], [nex_x, ny], dia.beg_x));
            if next_track_cond {
                line.nodes.push([nex_x, ny])
            }
        } else if nex_time == dep_time {
            if next_track_cond {
                line.nodes.push([nex_x, ny])
            }
        } else if nex_time < data.end_time {
            if track_cond {
                line.nodes.push([dep_x, cy])
            }
            if next_track_cond {
                line.nodes.push([nex_x, ny])
            }
        } else {
            line.nodes
                .push(solve_y_point([dep_x, cy], [nex_x, ny], dia.end_x));
            refresh_lines(lines, line);
        }
    } else {
        if nex_time < data.beg_time {
            // do nothing
        } else if nex_time < data.end_time {
            refresh_lines(lines, line);
            line.nodes
                .push(solve_y_point([pdep_x, cy], [nex_x, ny], dia.beg_x));
            line.nodes.push([nex_x, ny])
        } else if nex_time < dep_time {
            refresh_lines(lines, line);
            line.nodes
                .push(solve_y_point([pdep_x, cy], [nex_x, ny], dia.beg_x));
            line.nodes
                .push(solve_y_point([pdep_x, cy], [nex_x, ny], dia.end_x));
            refresh_lines(lines, line);
        } else {
            // do nothing
        }
    }
}

fn add_extra_collisions(dia: &mut Diagram) {
    // add extra collisions
    dia.collisions.push(Collision {
        nodes: vec![
            [dia.beg_x, dia.beg_y - STATION_LABEL_PADDING],
            [dia.end_x, dia.beg_y - STATION_LABEL_PADDING],
            [dia.end_x, dia.beg_y - STATION_LABEL_PADDING - 10.0],
            [dia.beg_x, dia.beg_y - STATION_LABEL_PADDING - 10.0],
        ],
    });
    dia.collisions.push(Collision {
        nodes: vec![
            [dia.beg_x, dia.end_y + STATION_LABEL_PADDING],
            [dia.end_x, dia.end_y + STATION_LABEL_PADDING],
            [dia.end_x, dia.end_y + STATION_LABEL_PADDING + 10.0],
            [dia.beg_x, dia.end_y + STATION_LABEL_PADDING + 10.0],
        ],
    });
}

fn parse_diagram_extrema(dia: &mut Diagram) {
    // iterates over the diagram's collisions
    // then finds the min and max x and y values
    // and sets the diagram's beg_x, end_x, beg_y, end_y values
    // reset the values
    dia.beg_x = f64::MAX;
    dia.end_x = f64::MIN;
    dia.beg_y = f64::MAX;
    dia.end_y = f64::MIN;
    // iterate over the collisions
    for c in dia.collisions.iter() {
        for n in c.nodes.iter() {
            if n[0] < dia.beg_x {
                dia.beg_x = n[0];
            }
            if n[0] > dia.end_x {
                dia.end_x = n[0];
            }
            if n[1] < dia.beg_y {
                dia.beg_y = n[1];
            }
            if n[1] > dia.end_y {
                dia.end_y = n[1];
            }
        }
    }
}

fn parse_data(data: &RawDiagram) -> Diagram {
    // Parse the CBOR data into the Diagram struct
    // check if data is a map
    // let mut collision: Vec<u8> = Vec::new();
    let mut dia = Diagram::default();
    dia.beg_x = data.beg_time as f64 / 3600.0 * data.unit_length * data.time_axis_scale;
    dia.end_x = data.end_time as f64 / 3600.0 * data.unit_length * data.time_axis_scale;
    parse_stations(data, &mut dia);
    parse_trains(data, &mut dia);
    add_extra_collisions(&mut dia);
    parse_diagram_extrema(&mut dia);
    dia
}

#[wasm_func]
pub fn return_cbor(data: &[u8]) -> Vec<u8> {
    let data: RawDiagram = from_reader(data).unwrap();
    let parsed_data: Diagram = parse_data(&data);
    let mut output = Vec::new();
    match into_writer(&parsed_data, &mut output) {
        Ok(_) => output,
        Err(_) => panic!("Failed to write CBOR"),
    }
}
