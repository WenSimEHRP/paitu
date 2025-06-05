#let plg = plugin("paiagram_wasm.wasm")
#import "foreign/qetrc.typ": read-qetrc-2
#set page(width: auto, height: auto)
#let gen_trains(num, stations) = {
  let ret = ()
  for i in range(num) {
    let schedule = ()
    for j in range(stations) {
      schedule.push(((j, j + 1), (arr: j, dep: j + 1, tracks: 1)))
    }
    ret.push((i, (schedule: schedule)))
  }
  ret
}

#let gen_stations(num) = {
  let ret = ()
  for i in range(num) {
    ret.push((i, (tracks: 3)))
  }
  ret
}

#let gen_intervals(num) = {
  let ret = ()
  for i in range(num) {
    ret.push(((i, i + 1), (length: 1000)))
    ret.push(((i + 1, i), (length: 1000)))
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

#let a = gen_intervals_to_draw(50)
#let s = gen_stations(100)
#let i = gen_intervals(100)
#let t = gen_trains(1000, 100)

#let data = plg.process(
  cbor.encode((
    stations: s,
    intervals: i,
    train_numbers: t,
  )),
  cbor.encode((
    intervals_to_draw: a,
    stations_to_draw: none,
    draw_station_names: false,
    draw_labels: false,
    draw_hours: false,
    draw_collision: false,
    draw_heatmap: false,
    draw_tracks: true,
    draw_occupancy_map: false,
    position_axis_mode: "Auto",
    beg_hour: 0,
    end_hour: 24,
    position_axis_scale: 1.0,
    time_scale: 1.0,
    unit_length: 1cm / 1pt,
  )),
)

/*

#(data = cbor(data))

#context {
  block(
    width: (data.graph_collisions.x_max - data.graph_collisions.x_min) * 1pt,
    height: (data.graph_collisions.y_max - data.graph_collisions.y_min) * 1pt,
    stroke: 1pt + blue,
    {
      if "collisions" in data.graph_collisions {
        for col in data.graph_collisions.collisions {
          place(
            dx: data.graph_collisions.x_min * 1pt,
            dy: data.graph_collisions.y_min * -1pt,
            curve(
              stroke: 1pt + red,
              fill: red.transparentize(70%),
              curve.move((col.at(0).at(0) * 1pt, col.at(0).at(1) * 1pt)),
              ..col.map(it => curve.line((it.at(0) * 1pt, it.at(1) * 1pt))),
              curve.close(),
            ),
          )
        }
      }
      place(
        dx: data.graph_collisions.x_min * 1pt,
        dy: data.graph_collisions.y_min * -1pt,
        grid(
          columns: (1fr,) * 24,
          rows: data.grid_intervals.map(it => it * 1pt),
          stroke: 1pt + gray,
        ),
      )
    },
  )
}

#pagebreak()

#data

*/
