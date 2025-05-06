#import "elem/mod.typ" as elem
#import "utils.typ": *

/// Draws a train timetable diagram
///
/// - stations (dictionary): Stations to draw
/// - trains (dictionary): Trains to draw
/// - routings (dictionary): Routings to draw
/// - track-scale (int, float, auto, none): Scales track space for stations.
///     When set to `auto`, the track space is set to be 1em.
///     When set to `none`, extra tracks will not be drawn.
/// - track-space-scale (int, float, auto): Scales the space between stations
/// - track-space-scale-mode (str): Sets how to scale the space between stations.
///     possible values are `"linear"` , `uniformed`, and `"logarithmic"`
/// - track-stroke (stroke): How to draw the track lines
/// - track-numbering (str): Track identifier numbering.
///     The numbering is ignored when the station sets their own names.
/// - stroke (stroke): How to draw the frame
/// - beg (int): beginning time, in seconds.
///     Everything within `beg` and `end` will be drawn
/// - end (int): ending time, in seconds.
///     Everything within `beg` and `end` will be drawn
/// - background (none, color, gradient, tiling): How to draw the background
/// - reversed (boolean): Whether to draw the diagram reversely
/// - train-coloring (str, auto, none): Controls train coloring. When set to
///     `auto`, each train without a special color scheme will randomly
///     pick a color. When set to `"by-speed"`, trains will be colored by
///     its relative speed. When set to `none`, no special rules will apply.
/// - length-unit (string): Controls the unit of length displayed on the diagram.
///     Possible values are "km", "mi", "m", and "time".
/// - unit-length (length): The length of one unit
/// - horizontal-scale (int, float): Horizontal scale value
/// - debug (boolean): Super secret debug flick
/// -> content
#let paiagram(
  // basic information
  stations: (:),
  trains: (:),
  routings: (:),
  // station tracks
  track-scale: auto,
  track-space-scale: 1,
  track-space-scale-mode: "linear",
  track-stroke: stroke(thickness: 1pt, dash: "dashed", paint: gray, cap: "round"),
  track-numbering: "1",
  // border
  stroke: 1pt + gray,
  // range to draw
  beg: 0,
  end: 24 * 3600,
  // general
  background: none,
  reversed: false,
  train-coloring: auto,
  length-unit: "km",
  unit-length: 1cm,
  horizontal-scale: 2,
  debug: false,
  show-label: true,
) = context {
  let track-scale = track-scale
  if track-scale == auto {
    track-scale = 1em.to-absolute() / unit-length
  } else if track-scale == none {
    track-scale = 0
  }
  // simple checks
  let collision = ()
  // elements to render
  let (stations, max-height) = elem.stations.make-stations(
    stations,
    track-scale,
    track-space-scale,
    track-space-scale-mode,
    debug: false,
  )
  let (stat-elem, collision) = elem.stations.draw-stations(
    collision,
    stations,
    beg,
    end,
    unit-length,
    horizontal-scale,
    track-scale,
    track-stroke,
    track-numbering,
  )
  let (dia-elem, collision) = elem.diagram.draw-diagram(
    collision,
    beg,
    end,
    unit-length,
    horizontal-scale,
    stroke,
    max-height,
  )
  let (train-elem, collision) = elem.trains.make-trains(
    collision,
    trains,
    stations,
    beg,
    end,
    track-scale,
    unit-length,
    horizontal-scale,
    train-coloring,
    show-label,
    debug: debug,
  )
  let elements = {
    for e in stat-elem { e }
    for e in dia-elem { e }
    for e in train-elem { e }
    if debug {
      for col in collision {
        place(
          curve(
            fill: blue.transparentize(50%),
            stroke: blue + 1pt,
            curve.move((col.x.beg, col.y.beg)),
            curve.line((col.x.end, col.y.beg)),
            curve.line((col.x.end, col.y.end)),
            curve.line((col.x.beg, col.y.end)),
            curve.close(),
          ),
        )
        place(
          curve(
            stroke: (paint: blue, cap: "butt", join: "bevel"),
            curve.move((col.x.beg, col.y.beg)),
            curve.line((col.x.end, col.y.end)),
            curve.line((col.x.end, col.y.beg)),
            curve.line((col.x.beg, col.y.end)),
          ),
        )
        place(
          dx: col.x.beg - 2pt,
          dy: col.y.beg - 2pt,
          circle(
            stroke: none,
            fill: blue,
            radius: 2pt,
          )
        )
      }
    }
  }
  let xbeg = 0pt
  let ybeg = 0pt
  let xend = 0pt
  let yend = 0pt
  for col in collision {
    xbeg = calc.min(col.x.beg, xbeg)
    xend = calc.max(col.x.end, xend)
    ybeg = calc.min(col.y.beg, ybeg)
    yend = calc.max(col.y.end, yend)
  }
  box(
    width: xend - xbeg,
    height: yend - ybeg,
    stroke: if debug { blue + 1pt },
    place(dx: -xbeg, dy: -ybeg, elements),
  )
}
