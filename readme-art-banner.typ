#set page(
  width: (11cm * 1.618),
  height: 4cm,
  margin: 0cm,
)
#set par(justify: true)
#set text(font: "Glow Sans SC Extended") // https://github.com/welai/glow-sans
#place(image("examples/sample.png", width: 11cm * 1.618))
#place(
  rect(
    width: 100%,
    height: 100%,
    stroke: none,
    fill: gradient.linear(white.transparentize(20%), white.transparentize(100%), angle: -90deg),
  ),
)

#place(
  rect(
    width: 11cm,
    height: 100%,
    stroke: none,
    fill: gradient.linear(aqua.transparentize(80%), aqua.transparentize(100%), angle: 180deg),
  ),
)

#align(
  horizon,
  box(width: 10cm, inset: (left: 1.5cm))[
    #text(size: 30pt, weight: 100)[PAIAGRAM]\
    #text(size: 20pt, weight: 200)[派途]
  ],
)
