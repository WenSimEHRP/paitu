#let plg = plugin("paiagram_wasm.wasm")
#import "foreign/qetrc.typ": read-qetrc-2
// stations: Vec<(StationID, Track)>,
// intervals: Vec<(IntervalID, IntervalLength)>,
// trains: Vec<(TrainNumberID, Vec<(StationID, Schedule)>)>,

#let gen_trains(num) = {
  let ret = ()
  for i in range(num) {
    ret.push((
      i,
      (
        (0, (arr: 0, dep: 1)),
        (1, (arr: 0, dep: 1)),
        (2, (arr: 0, dep: 1)),
        (3, (arr: 0, dep: 1)),
        (4, (arr: 0, dep: 1)),
        (5, (arr: 0, dep: 1)),
      )
        * 10,
    ))
  }
  ret
}

#let gen_stations(num) = {
  let ret = ()
  for i in range(num) {
    ret.push((i, 1))
  }
  ret
}

#let gen_intervals(num) = {
  let ret = ()
  for i in range(num) {
    ret.push(((i, i + 1), (i + 1) * 1000))
  }
  ret
}

#let gen_intervals_to_draw(num) = {
  let ret = ()
  for i in range(num) {
    ret.push(((i, i + 1), true))
  }
  ret
}

#let data = plg.process(
  cbor.encode((
    "stations": gen_stations(50),
    "intervals": gen_intervals(50),
    "trains": gen_trains(100),
  )),
  cbor.encode((
    intervals_to_draw: gen_intervals_to_draw(50),
    stations_to_draw: none,
    draw_station_names: false,
    draw_labels: false,
    draw_hours: false,
    draw_collision: false,
    draw_heatmap: false,
    interval_scale_mode: "Auto",
    beg_hour: 0,
    end_hour: 24,
    scale: 1.0,
    unit_length: 1.0,
  )),
)

#cbor(data)
