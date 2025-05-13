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
    position_scale: "Logarithmic",
    time_scale: "Auto",
    beg_time: 0,
    end_time: 0,
    label_angle: 0.0,
    unit_length: 0.0,
    position_axis_scale: 0.2,
    track_spacing_scale: 0.0,
    time_axis_scale: 0.2,
  )),
)

#set page(width: auto, height: auto)
// draw a grid using a's info
#grid(
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
                      fill: red.transparentize(100% - d.at(i) * 15%),
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
)
