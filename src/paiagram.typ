#import "elem/mod.typ" as elem
#import "utils.typ": *

/// Draws a train timetable diagram
///
/// ```example
/// #import "@local/paiagram:0.1.0": paiagram
/// #paiagram(
///   beg: 6 * 3600,
///   end: 9 * 3600,
///   track-scale: .5,
///   track-space-scale: 3,
///   stations: (
///     "a": (position: 0, tracks: 1, name: "Station A"),
///     "b": (position: 1, tracks: 2, name: "Station B"),
///   ),
///   trains: (
///     "Arupha": (
///       schedule: (
///    (arrival_time: 6 * 3600, station: "a", track_index: 0),
///    (arrival_time: 7 * 3600, station: "b", track_index: 0),
///       )
///     ),
///     "Aleph": (
///       schedule: (
///    (arrival_time: 7 * 3600, station: "a", track_index: 0),
///    (arrival_time: 8 * 3600, station: "b", track_index: 0),
///       )
///     ),
///     "Shuke": (
///       schedule: (
///    (arrival_time: 7 * 3600, station: "b", track_index: 1),
///    (arrival_time: 8 * 3600, station: "a", track_index: 0),
///       )
///     ),
///     "Beita": (
///       schedule: (
///    (arrival_time: 7.5 * 3600, departure_time: 8 * 3600, station: "b", track_index: 1),
///    (arrival_time: 9 * 3600, station: "a", track_index: 0),
///       )
///     )
///   )
/// )
/// ```
///
/// -> content
#let paiagram(
  // basic information
  /// Stations to draw
  /// -> dictionary
  stations: (:),
  /// Trains to draw
  /// -> dictionary
  trains: (:),
  /// Routings to draw
  /// -> dictionary
  routings: (:),
  // station tracks
  /// Set the distance scale between tracks on the diagram.
  /// Setting it to `auto` will automatically calculate the distance based on the
  /// text size. Setting it to `none`  or `0` will remove the track space.
  /// -> auto | int | float | none
  track-scale: auto,
  /// Set the distance scale between stations on the diagram.
  /// ```example
  /// #import "@local/paiagram:0.1.0": paiagram
  /// #paiagram(
  ///   beg: 6 * 3600,
  ///   end: 9 * 3600,
  ///   stations: (
  ///     "a": (position: 0),
  ///     "b": (position: 1),
  ///   )
  /// )
  /// ```
  /// ```example
  /// #import "@local/paiagram:0.1.0": paiagram
  /// #paiagram(
  ///   beg: 6 * 3600,
  ///   end: 9 * 3600,
  ///   track-space-scale: 4.2,
  ///   stations: (
  ///     "a": (position: 0),
  ///     "b": (position: 1),
  ///   )
  /// )
  /// ```
  /// -> int | float
  track-space-scale: 1,
  /// How to scale the track space.
  /// Possible values are:
  /// - `"linear"`: The track space is scaled linearly.
  /// - `"uniform"`: Each track space are the same size.
  /// - `"Logarithmic"`: The track space is scaled logarithmically.
  /// - `"sqrt"`: The track space is scaled by the square root.
  /// -> string
  track-space-scale-mode: "linear",
  /// The track space stroke.
  /// -> none | auto | length | color | gradient | stroke | tiling | dictionary
  track-stroke: stroke(thickness: 1pt, dash: "dashed", paint: gray, cap: "round"),
  /// Numbering scheme for the tracks.
  /// -> string
  track-numbering: "1",
  /// How to stroke the diagram border.
  /// -> none | auto | length | color | gradient | stroke | tiling | dictionary
  stroke: 1pt + gray,
  /// Beginning time of the diagram, in seconds.
  /// -> int
  beg: 0,
  /// End time of the diagram, in seconds.
  /// -> int
  end: 24 * 3600,
  /// How to fill the diagram.
  /// -> none | color | gradient | tiling
  fill: none,
  /// Whether to draw the diagram reversed.
  /// -> bool
  reversed: false,
  /// Train coloring mode.
  /// When set to `auto`, the train will be colored based on the train name.
  /// When set to `none`, all trains will be colored grey.
  /// When set to `"default"`, the trains will be colored based on its stroke settings.
  /// -> none | auto | length | color | gradient | stroke | tiling | dictionary | array
  train-coloring: auto,
  /// Length unit for the diagram.
  /// Possible values are:
  /// - `"km"`: kilometers
  /// - `"mi"`: miles
  ///  - `"time`: hours
  /// -> string
  length-unit: "km",
  /// unit length for the diagram.
  /// -> length
  unit-length: 1cm,
  /// Horizontal scale for the diagram.
  /// -> int | float
  horizontal-scale: 2,
  /// Whether to draw the diagram in debug mode.
  /// -> bool
  debug: false,
  /// Whether to show the train labels.
  /// -> bool
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
