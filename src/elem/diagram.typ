#import "../utils.typ": *

/// Draws the diagram
///
/// - collision (array): Global collision array
/// - beg (ing): Diagram beginning time
/// - end (ing): Diagram end time
/// - u (length): Unit length
/// - h (h): Horizontal scale
/// - stroke (stroke): How to stroke the borders
/// - max-height (length): Maximum height of the diagram
/// -> array
#let draw-diagram(
  collision,
  beg,
  end,
  u,
  h,
  stroke,
  max-height,
) = {
  let elem = ()
  // borders
  elem.push(
    place(
      curve(
        stroke: stroke,
        curve.line((0pt, -1 * u)),
        curve.line((to-x(end - beg) * u * h, -1 * u)),
        curve.line((to-x(end - beg) * u * h, (max-height + 1) * u)),
        curve.line((0pt, (max-height + 1) * u)),
        curve.close(),
      ),
    ),
  )
  // north-west label
  let c = to-datetime(s: beg).display("[hour]:[minute]")
  let m = measure(c)
  elem.push(
    place(
      dx: -m.width * .5,
      dy: -1 * u - m.height - 3pt,
      c,
    ),
  )
  collision.push((
    x: (beg: -m.width * .5, end: m.width * .5),
    y: (beg: -1 * u - m.height - 3pt, end: -1 * u - 3pt),
  ))
  // south=west label
  elem.push(
    place(
      dx: -m.width * .5,
      dy: (max-height + 1) * u + 3pt,
      c,
    ),
  )
  collision.push((
    x: (beg: -m.width * .5, end: m.width * .5),
    y: (beg: (max-height + 1) * u + 3pt, end: (max-height + 1) * u + m.height + 3pt),
  ))
  // north-east label
  let c = to-datetime(s: end).display("[hour]:[minute]")
  let m = measure(c)
  elem.push(
    place(
      dx: to-x(end - beg) * u * h - m.width * .5,
      dy: -1 * u - m.height - 3pt,
      c,
    ),
  )
  collision.push((
    x: (beg: to-x(end - beg) * u * h - m.width * .5, end: to-x(end - beg) * u * h + m.width * .5),
    y: (beg: -1 * u - m.height - 3pt, end: -1 * u - 3pt),
  ))
  // south-east label
  elem.push(
    place(
      dx: to-x(end - beg) * u * h - m.width * .5,
      dy: (max-height + 1) * u + 3pt,
      c,
    ),
  )
  collision.push((
    x: (beg: to-x(end - beg) * u * h - m.width * .5, end: to-x(end - beg) * u * h + m.width * .5),
    y: (beg: (max-height + 1) * u + 3pt, end: (max-height + 1) * u + m.height + 3pt),
  ))
  let l = int((end - beg) / 3600) + 1
  if calc.rem(beg, 3600) == 0 {
    l -= 1
  }
  for i in range(1, l) {
    let c = to-datetime(s: i * 3600 + beg).display("[hour]")
    let m = measure(c)
    let x = (i - to-x(calc.rem(beg, 3600))) * 1 * u * h
    elem.push(
      place(
        dx: -m.width * .5 + x,
        dy: -1 * u - m.height - 3pt,
        c,
      ),
    )
    collision.push((
      x: (beg: x - m.width * .5, end: x + m.width * .5),
      y: (beg: -1 * u - m.height - 3pt, end: -1 * u - 3pt),
    ))
    collision.push((
      x: (beg: x - m.width * .5, end: x + m.width * .5),
      y: (beg: (max-height + 1) * u + 3pt, end: (max-height + 1) * u + m.height + 3pt),
    ))
    elem.push(
      place(
        dx: -m.width * .5 + x,
        dy: (max-height + 1) * u + 3pt,
        c,
      ),
    )
    elem.push(
      place(
        curve(
          stroke: stroke,
          curve.move((x, -1 * u)),
          curve.line((x, (max-height + 1) * u)),
        ),
      ),
    )
  }
  // label between
  (elem, collision)
}
