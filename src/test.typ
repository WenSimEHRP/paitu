#let plg = plugin("paiagram_wasm.wasm")
#import "foreign/qetrc.typ": read-qetrc-2
#let wasm_process(info) = cbor(plg.process(info))

#let (s, t) = read-qetrc-2(json("../jinghu.pyetgr"))

#let a = wasm_process(
  cbor.encode((
    trains: t,
    stations: s,
    polygons: (:),
    routings: (:),
    position_scale: "Uniform",
    time_scale: "Linear",
    beg_hour: 0,
    end_hour: 0,
    label_angle: 0.0,
    unit_length: 0.0,
    position_axis_scale: 1.0,
    track_spacing_scale: 0.0,
    time_axis_scale: 2.0,
  )),
)

#let distr(s, w: auto) = {
  block(
    width: w,
    stack(
      dir: ltr,
      ..s.clusters().map(x => [#x]).intersperse(1fr),
    ),
  )
}

#set page(width: auto, height: auto)
#set text(font: "Sarasa Mono SC")
#set par(justify: true)
// draw a grid using a's info
#context {
  grid(
    columns: 2,
    rows: 2,
    gutter: 1em,
    grid(
      rows: a.station_info.stations.pairs().map(i => i.at(1).relative_y * 1cm),
      align: bottom + right,
      ..a
        .station_info
        .stations
        .pairs()
        .map(i => move(
          dy: measure(i.at(0)).height / 2,
          distr(i.at(0)),
        )),
    ),
    grid(
      columns: a.station_info.scales.map(i => i * 1cm),
      stroke: 1pt,
      rows: a.station_info.stations.pairs().map(i => i.at(1).relative_y * 1cm),
      ..{
        for (id, info) in a.station_info.stations.pairs() {
          for d in info.density {
            (
              grid.cell(
                grid(
                  columns: (1fr,) * 6,
                  rows: 100%,
                  ..{
                    for i in range(6) {
                      (
                        grid.cell(
                          x: i,
                          y: 0,
                          fill: blue.transparentize((1 - d.at(i) / a.station_info.max_density) * 100%),
                          [],
                        ),
                      )
                    }
                  }
                ),
              ),
            )
          }
        }
      }
    ),

    [],
    grid(
      rows: 1,
      columns: a.station_info.scales.map(i => i * 1cm) + (0pt,),
      ..range(25).map(it => move(
        dx: -measure(datetime(hour: calc.rem(it, 24), minute: 0, second: 0).display("[hour]:[minute]")).width / 2,
        datetime(hour: calc.rem(it, 24), minute: 0, second: 0).display("[hour]:[minute]"),
      ))
    ),
  )
}
#let paiagram() = { }
