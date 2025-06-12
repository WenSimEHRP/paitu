#import "../utils.typ": *
#let to-timestamp(h, m, s) = {
  return int(h * 3600 + m * 60 + s)
}

/// Try to match a qETRC default color
///
/// - type (str): Train type
/// -> color
#let _match-color(type) = {
  if type == "高速" { return color.rgb("#FF00BE") }
  if type == "动车组" { return color.rgb("#804000") }
  if type == "动车" { return color.rgb("#804000") }
  if type == "城际" { return color.rgb("FF33CC") }
  if type == "市郊" { return color.rgb("852EFF") }
  if type == "快速" { return color.rgb("FF0000") }
  if type == "特快" { return color.rgb("0000FF") }
  if type == "直达特快" { return color.rgb("FF00FF") }
  if type == "临客" { return color.rgb("#808080") }
  // 旅游
  // 通勤
  // 路用
  // 试运转
  // 补机
  // 行包
  // 直达
  // 直货
  // 班列
  // 特快行包
  // 普快
  // 普客
  // 摘挂
  // 小运转
  // 客车底
  // 单机
  // 区段
  // 动检
  color.rgb("#008000")
}

/// Reads a qETRC/pyETRC diagram file and returns the stations, trains, and routings.
///
/// - qetrc (dictionary):
/// -> (lines, trains, routings)
#let read-qetrc(qetrc) = {
  let stations = (:)
  let trains = (:)
  let routings = none
  for train in qetrc.at("trains") {
    let name = train.at("checi").at(0)
    let departure = train.at("sfz")
    let terminal = train.at("zdz")
    let schedule = ()
    // qETRC supports types, and we can add custom colorings
    let stroke = (
      thickness: train.at("UI").at("thickness", default: 1) * 1pt,
      paint: if "UI" in train and "Color" in train.UI { color.rgb(train.UI.Color) } else {
        // use a custom matching function
        if "type" not in train { red }
        _match-color(train.type)
      },
    )
    for station in train.at("timetable") {
      let station_name = station.at("zhanming")
      let arrival_time = to-timestamp(..station.at("ddsj").split(":").map(int))
      let departure_time = to-timestamp(..station.at("cfsj").split(":").map(int))
      schedule.push((
        station: station_name,
        arrival_time: arrival_time,
        departure_time: departure_time,
        track_index: 0,
      ))
    }
    trains.insert(
      name,
      (
        stroke: stroke,
        departure: departure,
        terminal: terminal,
        schedule: schedule,
      ),
    )
  }
  for station in qetrc.at("line").at("stations") {
    let name = station.at("zhanming")
    let tracks = 1
    let pos = station.at("licheng")
    stations.insert(
      name,
      (
        tracks: 1,
        position: pos,
      ),
    )
  }
  (stations, trains, routings)
}

#let read-qetrc-2(qetrc) = {
  let stations = (:)
  let trains = (:)
  for train in qetrc.at("trains") {
    let name = train.at("checi").at(0)
    let departure = train.at("sfz")
    let terminal = train.at("zdz")
    let schedule = ()
    for station in train.at("timetable") {
      let station_name = station.at("zhanming")
      let time = (
        to-timestamp(..station.at("ddsj").split(":").map(int)),
        to-timestamp(..station.at("cfsj").split(":").map(int)),
      )
      schedule.push((
        station: station_name,
        time: time,
        track: 0,
      ))
    }
    trains.insert(
      name,
      (
        schedule: schedule,
      ),
    )
  }
  for station in qetrc.at("line").at("stations") {
    let name = station.at("zhanming")
    let tracks = 1
    let pos = station.at("licheng")
    stations.insert(
      name,
      (
        tracks: 1,
        position: float(pos),
      ),
    )
  }
  (stations, trains)
}
