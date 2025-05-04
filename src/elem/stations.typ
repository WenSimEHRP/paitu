#import "../utils.typ": *
#import "../deps.typ": *

/// Preprocess the stations and return maximum draw height
///
/// - stations (dictionary): Stations to process
/// - track-scale (int, float): Track scale
/// - track-space-scale (int, float): Track space (space between stations) scale
/// - track-space-scale-mode (str): How to scale the track space
/// - debug (boolean): Debug mode flick
/// -> array
#let make-stations(
  stations,
  track-scale,
  track-space-scale,
  track-space-scale-mode,
  debug: false,
) = {
  assert(stations.len() > 0, message: "There should be at least one station")
  let ret = (:)
  let a = stations.pairs().sorted(key: it => it.at(1).position)
  let draw-height = 0
  let prev-pos = a.at(0).at(1).position
  let next-pos = 0
  for i in range(a.len()) {
    let name = a.at(i).at(0)
    let info = a.at(i).at(1)
    let track_count = 0
    if type(info.tracks) == array {
      track_count = info.tracks.len() - 1
    } else {
      track_count = info.tracks - 1
    }
    let l = (info.position - prev-pos)
    if track-space-scale-mode == "log" and l != 0 {
      l = calc.log(l, base: 2)
    }
    if track-space-scale-mode == "sqrt" {
      l = calc.sqrt(l)
    }
    draw-height += l * track-space-scale + track-scale * track_count
    prev-pos = info.position
    let neighbors = ()
    if i != 0 { neighbors.push(a.at(i - 1).at(0)) }
    if i != a.len() - 1 { neighbors.push(a.at(i + 1).at(0)) }
    ret.insert(
      name,
      (
        neighbors: neighbors,
        draw-height: draw-height,
        ..info,
      ),
    )
  }
  return (ret, draw-height)
}

/// Draw the stations on the diagram
///
/// - collision (array): Global collision array
/// - stations (dictionary): Stations to process
/// - beg (int): Diagram beginning time
/// - end (int): Diagram end time
/// - u (length): Unit length
/// - h (int, float): Horizontal scale
/// - track-scale (int, float): Track scale
/// - track-stroke (stroke): How to stroke the tracks
/// - track-numbering (str): How to number the tracks
/// -> array
#let draw-stations(
  collision,
  stations,
  beg,
  end,
  u,
  h,
  track-scale,
  track-stroke,
  track-numbering,
) = {
  let elem = ()
  for stat in stations {
    let info = stat.at(1)
    let c = none
    let t = if type(info.tracks) == array { info.tracks.len() } else { info.tracks }
    if track-scale == 0 {
      t = 1
    }
    let y = info.draw-height * u
    let e = to-x(end - beg) * u * h

    let station_label = [#grid(
        columns: (auto, 4em),
        gutter: .5em,
        align: right,
        [#(info.at("name", default: stat.at(0)))], [#zero.num(info.position, digits: 2, math: false)],
      )]
    let m = measure(station_label)
    let x-offset = if track-scale != 0 { 3em.to-absolute() } else { 3pt }
    c = place(
      dx: -m.width - x-offset,
      dy: y - m.height * .5,
      station_label,
    )
    elem.push(c)
    collision.push((
      x: (beg: -m.width - x-offset, end: -x-offset),
      y: (beg: y - m.height * .5, end: y + m.height * .5),
    ))
    for i in range(t) {
      let y = y - i * track-scale * u
      c = place(
        curve(
          stroke: track-stroke,
          curve.move((0pt, y)),
          curve.line((e, y)),
        ),
      )
      elem.push(c)
      if track-scale != 0 {
        let track-name = if type(info.tracks) == array { [#info.tracks.at(i)] } else {
          [#numbering(track-numbering, i + 1)]
        }
        let m = measure[#track-name]
        let start_dx = -m.width - 3pt
        c = place(
          dx: start_dx,
          dy: y - m.height / 2,
          [#track-name],
        )
        elem.push(c)
        c = place(
          dx: to-x(end - beg) * u * h + 3pt,
          dy: y - m.height / 2,
          [#track-name],
        )
        elem.push(c)
        collision.push((
          x: (beg: -m.width - 3pt, end: -3pt),
          y: (beg: -m.height * .5 + y, end: m.height * .5 + y),
        ))
        collision.push((
          x: (beg: to-x(end - beg) * u * h + 3pt, end: to-x(end - beg) * u * h + 3pt + m.width),
          y: (beg: -m.height * .5 + y, end: m.height * .5 + y),
        ))
      }
    }
  }
  (elem, collision)
}
