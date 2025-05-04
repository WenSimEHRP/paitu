
/// Converts a time to a timestamp
///
/// - h (int, float): hour
/// - m (int, float): minute
/// - s (int, float): second
/// -> int
#let to-timestamp(h: 0, m: 0, s: 0) = {
  return int(h * 3600 + m * 60 + s)
}

/// Converts a timestamp to a datetime object
///
/// - h (int, float): hour
/// - m (int, float): minute
/// - s (int, float): second
/// -> datetime
#let to-datetime(h: 0, m: 0, s: 0) = {
  m += calc.floor(s / 60)
  s = calc.rem(s, 60)
  h += calc.floor(m / 60)
  m = calc.rem(m, 60)
  h = calc.rem(h, 24)
  return datetime(hour: int(h), minute: int(m), second: int(s))
}

/// Converts timestamp to x-position
///
/// - time (int): timestamp
/// -> float, int
#let to-x(time) = {
  return time / 3600
}

/// Return a station's y-position
///
/// - stations (dict): dictionary of stations
/// - name (str): station to get the y-position of
/// -> float
#let get-station-y(stations, name) = {
  return stations.at(name).at("draw-height")
}

/// Returns a station's neighbors
///
/// - stations (dictionary): Station list
/// - name (str): Station to look up
/// -> array, none
#let get-station-neighbors(stations, name) = {
  return stations.at(name).at("neighbors", default: none)
}

/// Return a station's track count
///
/// - stations (dict): dictionary of stations
/// - name (str): station to get the track number
/// -> int
#let get-station-tracks(stations, name) = {
  let t = stations.at(name).at("tracks")
  if type(t) == array {
    return t.len()
  } else {
    return t
  }
}

/// Solve for a y value given two points and an x value
///
/// - p1 (array): first point
/// - p2 (array): second point
/// - x (int): x input
/// -> array, none
#let solve-y(p1, p2, x) = {
  if p1.at(0) == p2.at(0) {
    if p1.at(0) == x {
      return (x, p1.at(1))
    } else {
      return none
    }
  }
  let slope = (p2.at(1) - p1.at(1)) / (p2.at(0) - p1.at(0))
  let b = p1.at(1) - slope * p1.at(0)
  let y = slope * x + b
  return (x, y)
}

/// Gets polar, returns x, y coordinates
///
/// - p0 (array): origin
/// - r (int, float, length): radius
/// - theta (degree): angle
/// -> array
#let rec(p0: (0, 0), p1) = {
  let x = p0.at(0) + p1.r * calc.cos(p1.theta)
  let y = p0.at(1) + p1.r * calc.sin(p1.theta)
  return (x, y)
}

/// Gets x, y coordinates, returns polar
///
/// - p0 (array): origin
/// - p1 (array): point
/// -> dictionary
#let pol(p0, p1) = {
  let dx = p1.at(0) - p0.at(0)
  let dy = p1.at(1) - p0.at(1)
  let r = calc.sqrt(dx * dx + dy * dy)
  let theta = calc.atan2(dy, dx)
  return (r: r, theta: theta)
}
