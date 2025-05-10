#import "./foreign/qetrc.typ": read-qetrc
#let example-plugin = plugin("paiagram_wasm.wasm")

#let to-timestamp(h, m, s) = {
  return int(h * 3600 + m * 60 + s)
}

#let debug = true

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
    // let stroke = (
    //   thickness: train.at("UI").at("thickness", default: 1) * 1pt,
    //   paint: if "UI" in train and "Color" in train.UI { color.rgb(train.UI.Color) } else {
    //     // use a custom matching function
    //     if "type" not in train { red }
    //     _match-color(train.type)
    //   },
    // )
    for station in train.at("timetable") {
      let station_name = station.at("zhanming")
      let arrival_time = to-timestamp(..station.at("ddsj").split(":").map(int))
      let departure_time = to-timestamp(..station.at("cfsj").split(":").map(int))
      schedule.push((
        id: station_name,
        time: (arrival_time, departure_time),
        track: 0,
      ))
    }
    trains.insert(
      name,
      (
        // stroke: stroke,
        // departure: departure,
        // terminal: terminal,
        label: measure([#name]).values().map(it => float(it / 1pt)),
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
        label: measure([#name]).values().map(it => float(it / 1pt)),
        position: float(pos),
      ),
    )
  }
  (stations, trains, routings)
}


#context {
  [
    #let (qstations, qtrains, qroutings) = read-qetrc(json("../jingha.pyetgr"))
    #let unit_length = 1cm
    #let stuff-to-draw = cbor(
      example-plugin.return_cbor(
        cbor.encode((
          stations: qstations,
          trains: qtrains,
          routings: (),
          polygons: (),
          angle: 0.0,
          station_scale_mode: "Logarithmic",
          label_angle: 0.0,
          unit_length: unit_length / 1pt,
          position_axis_scale: .6,
          track_spacing_scale: .8em.to-absolute() / 1pt,
          time_axis_scale: 2.0,
          beg_time: 0,
          end_time: 24 * 3600,
        )),
      ),
    )

    #set page(width: auto, height: auto, margin: 1in)
    #set text(font: "IBM Plex Sans SC")
    #show raw: set text(font: "Sarasa Mono SC")

    #let data = ()

    #box(
      width: (stuff-to-draw.end_x - stuff-to-draw.beg_x) * 1pt,
      height: (stuff-to-draw.end_y - stuff-to-draw.beg_y) * 1pt,
      stroke: blue,
    )[#place(
        dx: stuff-to-draw.beg_x * -1pt,
        dy: stuff-to-draw.beg_y * -1pt,
      )[#{
          let grid_stuff = ()
          for (id, stat) in stuff-to-draw.stations.pairs().sorted(key: it => it.at(1).draw_height) {
            grid_stuff.push(stat.rel_height * 1pt)
          }
          place(
            box(
              width: 100% + stuff-to-draw.beg_x * 1pt,
              grid(
              rows: grid_stuff,
              columns: (1fr, .5cm) * 24,
              stroke: 1pt,
              align: center + horizon,
            ),)
          )

          for (id, train) in stuff-to-draw.trains {
            for line in train.lines {
              let first_node = line.nodes.at(0)
              let col = oklch(75%, 25%, calc.rem(train.rand, 360) * 1deg)
              place(
                curve(
                  stroke: (paint: col, cap: "round", join: "round"),
                  curve.move(first_node.map(x => x * 1pt)),
                  ..line.nodes.map(it => curve.line(it.map(x => x * 1pt))),
                ),
              )

              if not debug {
                continue
              }

              for (i, node) in line.nodes.enumerate() {
                place(
                  dx: node.at(0) * 1pt - 2pt,
                  dy: node.at(1) * 1pt - 2pt,
                  circle(
                    radius: 2pt,
                    fill: col,
                  ),
                )
                place(
                  dx: node.at(0) * 1pt + 2pt,
                  dy: node.at(1) * 1pt + 2pt,
                  text(font: "Sarasa Mono SC", fill: col)[#i],
                )
              }
            }
          }

          if debug {
            for col in stuff-to-draw.collisions {
              let first_node = col.nodes.at(0)
              place(
                curve(
                  stroke: blue + 1pt,
                  fill: blue.transparentize(70%),
                  curve.move(first_node.map(x => x * 1pt)),
                  ..col.nodes.map(it => curve.line(it.map(x => x * 1pt))),
                  curve.close(),
                ),
              )
              place(
                dx: first_node.at(0) * 1pt - 2pt,
                dy: first_node.at(1) * 1pt - 2pt,
                circle(
                  radius: 2pt,
                  fill: blue,
                ),
              )
            }
          }
        }]]

    // #pagebreak()
    // #data.sorted(key: it => it.at(0).at(1))

    /*
    #place(
      top + right,
      rect(
        width: 1cm,
        height: 2cm,
        fill: red.transparentize(70%),
      ),
    )

    #place(
      top + right,
      dx: -.5cm,
      dy: .5cm,
      rect(
        width: 1cm,
        height: 2cm,
        fill: red.transparentize(70%),
      ),
    )
    */

  ]
}
