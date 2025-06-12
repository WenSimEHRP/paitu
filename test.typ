#import "src/lib.typ": *
#import "src/foreign/qetrc.typ": *
#set page(width: auto, height: auto)


#set text(font: "Sarasa Mono SC")

#let options = (
  track-space-scale-mode: "logarithmic",
  track-space-scale: .5,
  track-scale: none,
  track-numbering: "あ",
  debug: false,
  horizontal-scale: 2,
  beg: 0 * 3600,
  end: 24 * 3600,
  unit-length: 1cm,
  train-coloring: auto,
  show-label: false,
)
#grid(gutter: .5em, columns: 2, align: left)[
  // #let (qstations, qtrains, qroutings) = read-qetrc(json("jinghu.pyetgr"))
  // #paiagram(
  //   stations: qstations,
  //   trains: qtrains,
  //   ..options,
  // )
][
  #let (qstations, qtrains, qroutings) = read-qetrc(json("jinghu.pyetgr"))
  #paiagram(
    stations: qstations,
    trains: qtrains,
    ..options,
  )

  // #let (qstations, qtrains, qroutings) = read-qetrc(json("jingguang.pyetgr"))
  // #paiagram(
  //   stations: qstations,
  //   trains: qtrains,
  //   ..options,
  // )

  // #let (qstations, qtrains, qroutings) = read-qetrc(json("examples/sample.pyetgr"))
  // #paiagram(
  //   stations: qstations,
  //   trains: qtrains,
  //   ..options,
  // )
]
