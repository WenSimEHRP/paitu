#import "./foreign/qetrc.typ": read-qetrc
#let example-plugin = plugin("paiagram_wasm.wasm")

#let to-timestamp(h, m, s) = {
  return int(h * 3600 + m * 60 + s)
}

#let debug = false

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
        track: 1,
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
        tracks: calc.max(int(name.len() / 3), 1) + 1,
        label: measure([#name]).values().map(it => float(it / 1pt)),
        position: float(pos),
      ),
    )
  }
  (stations, trains, routings)
}


#context {
  [
    #let (qstations, qtrains, qroutings) = read-qetrc(json("../examples/sample.pyetgr"))
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
          track_spacing_scale: 1em.to-absolute() / 1pt,
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
          let grid_lines = ()
          for (i, (id, stat)) in stuff-to-draw.stations.pairs().sorted(key: it => it.at(1).draw_height).enumerate() {
            if i != 0 { grid_stuff.push(stat.rel_height * 1pt) }
            grid_stuff.push((stat.tracks - 1) * (1em.to-absolute() / 1pt) * 1pt)
            for j in range(24 * 6) {
              grid_lines.push(
                grid.cell(
                  x: j,
                  y: i * 2,
                  inset: 0pt,
                  grid(
                    columns: 1fr,
                    rows: (1fr,) * (stat.tracks - 1),
                    align: left + horizon,
                    inset: 2pt,
                    ..range(stat.tracks - 2).map(it => grid.hline(
                      y: it + 1,
                      stroke: (paint: gray, dash: "dotted", cap: "round"),
                    )),
                  ),
                ),
              )
            }
          }
          place(
            box(
              width: (86400 - 0) * 1cm * 2 / 3600,
              grid(
                rows: grid_stuff,
                columns: (1fr,) * 24 * 6,
                align: center + horizon,
                ..range(25).map(it => grid.vline(x: it * 6, stroke: gray)),
                // ..range(24)
                //   .map(i => range(5).map(j => grid.vline(
                //     x: j + i * 6 + 1,
                //     stroke: (paint: gray, dash: "dotted", cap: "round"),
                //   )))
                //   .flatten(),
                ..range(grid_stuff.len() + 1).map(it => grid.hline(y: it, stroke: gray)),
                ..range(24)
                  .map(i => range(int(grid_stuff.len() / 2)).map(j => grid.cell(
                    x: i * 6,
                    y: 1 + j * 2,
                    align: top + left,
                    inset: 0pt,
                    box(stroke: 1pt, rotate(40deg, reflow: true, text(size: 12em, fill: gray, font: "Noto Sans")[#i])),
                  )))
                  .flatten(),
                ..grid_lines,
              ),
            ),
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
