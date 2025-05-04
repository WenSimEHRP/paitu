#import "../src/lib.typ": *
#import "../src/foreign/qetrc.typ": *
#set page(width: auto, height: auto)


#set text(font: "Sarasa Mono SC")

#let options = (
  track-space-scale-mode: "log",
  track-space-scale: .5,
  track-scale: none,
  debug: false,
  horizontal-scale: 5,
  beg: 0 * 3600,
  end: 24 * 3600,
  unit-length: 1cm,
  train-coloring: "default",
  show-label: true,
)

#let (qstations, qtrains, qroutings) = read-qetrc(json("sample.pyetgr"))
#paiagram(
  stations: qstations,
  trains: qtrains,
  ..options,
)
