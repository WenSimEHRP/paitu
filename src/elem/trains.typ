#import "../utils.typ": *
#import "../deps.typ": *

/// Clears the current node group and push it into global node group
///
/// - node_group (array): Global node group
/// - nodes (): Current node group
/// -> array
#let _refresh-nodes(node_group, nodes) = {
  node_group.push(nodes)
  nodes = ()
  (node_group, nodes)
}

#let MAX_LOOP = 10

/// Gets current station's nodes
///
/// - cs (dictionary): Current station
/// - beg_time (int): Diagram beginning time
/// - end_time (int): Diagram ending time
/// - node_group (array): Global node group
/// - nodes (array): Current node group
/// - stat (dictionary): Stations
/// - track_scale (int, float): Track scale
/// -> array
#let _get-current-station-nodes(cs, beg_time, end_time, node_group, nodes, stat, track_scale) = {
  if cs.station not in stat { return ((), ()) }
  let cy = get-station-y(stat, cs.station)
  let track_cond = track_scale != 0 and cs.at("track_index", default: 0) > 0
  let cty = cy - cs.at("track_index", default: 0) * track_scale
  let arr_time = calc.rem(cs.arrival_time, 24 * 3600)
  let dep_time = calc.rem(cs.at("departure_time", default: cs.arrival_time), 24 * 3600)
  let y = if track_cond { cty } else { cy }
  if arr_time < beg_time {
    if dep_time < beg_time {
      // do nothing
    } else if dep_time < end_time {
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push((beg_time, y))
      nodes.push((dep_time, y))
    } else {
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push((beg_time, y))
      nodes.push((end_time, y))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
    }
  } else if arr_time < end_time {
    if dep_time < beg_time {
      nodes.push((arr_time, y))
      nodes.push((end_time, y))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
    } else if dep_time < arr_time {
      nodes.push((arr_time, y))
      nodes.push((end_time, y))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push((beg_time, y))
      nodes.push((dep_time, y))
    } else if dep_time == arr_time {
      nodes.push((arr_time, y))
    } else if dep_time < end_time {
      nodes.push((arr_time, y))
      nodes.push((dep_time, y))
    } else {
      nodes.push((arr_time, y))
      nodes.push((end_time, y))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
    }
  } else {
    if dep_time < beg_time {
      // do nothing
    } else if dep_time < end_time {
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push((beg_time, y))
      nodes.push((dep_time, y))
    } else if dep_time < arr_time {
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push((beg_time, y))
      nodes.push((end_time, y))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
    } else {
      // do nothing
    }
  }
  (node_group, nodes)
}

/// Gets next station's nodes
///
/// - cs (dictionary): Current station
/// - ns (dictionary): Next station
/// - beg_time (int): Diagram beginning time
/// - end_time (int): Diagram ending time
/// - node_group (array): Global node group
/// - nodes (array): Current node group
/// - stat (dictionary): Stations
/// - track_scale (int, float): Track scale
/// -> array
#let _get-next-station-nodes(cs, ns, beg_time, end_time, node_group, nodes, stat, track_scale) = {
  if cs.station not in stat or ns.station not in get-station-neighbors(stat, cs.station) or ns.station not in stat {
    return ((), ())
  }
  let cy = get-station-y(stat, cs.station)
  let ny = get-station-y(stat, ns.station) - (get-station-tracks(stat, ns.station) - 1) * track_scale
  if ny < cy and track_scale != 0 {
    cy = get-station-y(stat, cs.station) - (get-station-tracks(stat, cs.station) - 1) * track_scale
    ny = get-station-y(stat, ns.station)
  }
  let track_cond = (
    track_scale != 0
      and (
        if cy < ny {
          cs.track_index > 0
        } else {
          cs.track_index + 1 < get-station-tracks(stat, cs.station)
        }
      )
  )
  let next_track_cond = (
    track_scale != 0
      and (
        if cy < ny {
          ns.track_index + 1 < get-station-tracks(stat, ns.station)
        } else {
          ns.track_index > 0
        }
      )
  )
  let dep_time = calc.rem(cs.at("departure_time", default: cs.arrival_time), 24 * 3600)
  let nex_time = calc.rem(ns.arrival_time, 24 * 3600)
  let pdep_time = dep_time - 24 * 3600
  let nnex_time = nex_time + 24 * 3600
  if dep_time < beg_time {
    if nex_time < beg_time {
      // do nothing
    } else if nex_time < end_time {
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push(solve-y((dep_time, cy), (nex_time, ny), beg_time))
      nodes.push((nex_time, ny))
    } else {
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push(solve-y((dep_time, cy), (nex_time, ny), beg_time))
      nodes.push(solve-y((dep_time, cy), (nex_time, ny), end_time))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
    }
  } else if dep_time < end_time {
    if nex_time < beg_time {
      if track_cond { nodes.push((dep_time, cy)) }
      nodes.push(solve-y((dep_time, cy), (nnex_time, ny), end_time))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
    } else if nex_time < dep_time {
      if track_cond { nodes.push((dep_time, cy)) }
      nodes.push(solve-y((dep_time, cy), (nnex_time, ny), end_time))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push(solve-y((pdep_time, cy), (nex_time, ny), beg_time))
      if next_track_cond { nodes.push((nex_time, ny)) }
    } else if nex_time == dep_time {
      if next_track_cond { nodes.push((nex_time, ny)) }
    } else if nex_time < end_time {
      if track_cond { nodes.push((dep_time, cy)) }
      if next_track_cond { nodes.push((nex_time, ny)) }
    } else {
      nodes.push(solve-y((dep_time, cy), (nex_time, ny), end_time))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
    }
  } else {
    if nex_time < beg_time {
      // do nothing
    } else if nex_time < end_time {
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push(solve-y((pdep_time, cy), (nex_time, ny), beg_time))
      nodes.push((nex_time, ny))
    } else if nex_time < dep_time {
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
      nodes.push(solve-y((pdep_time, cy), (nex_time, ny), beg_time))
      nodes.push(solve-y((pdep_time, cy), (nex_time, ny), end_time))
      (node_group, nodes) = _refresh-nodes(node_group, nodes)
    } else {
      // do nothing
    }
  }
  (node_group, nodes)
}


/// Process train nodes
///
/// - train (dictionary): Trains to process
/// - stat (dictionary): Stations
/// - beg_time (int): Diagram beginning time
/// - end_time (int): Diagram ending time
/// - track_scale (int, float):
/// -> array
#let _process-train-nodes(
  train,
  stat,
  beg_time,
  end_time,
  track_scale,
) = {
  let node_group = ()
  let nodes = ()
  let id = train.at(0)
  for i in range(train.at(1).schedule.len()) {
    (node_group, nodes) = _get-current-station-nodes(
      train.at(1).schedule.at(i),
      beg_time,
      end_time,
      node_group,
      nodes,
      stat,
      track_scale,
    )
    if i == train.at(1).schedule.len() - 1 {
      break
    }
    (node_group, nodes) = _get-next-station-nodes(
      train.at(1).schedule.at(i),
      train.at(1).schedule.at(i + 1),
      beg_time,
      end_time,
      node_group,
      nodes,
      stat,
      track_scale,
    )
  }
  node_group.push(nodes)
  node_group.filter(it => it.len() >= 2)
}

/// Checks if two ranges overlap
///
/// - r1 (dictionary): Range 1
/// - r2 (dictionary): Range 2
/// -> boolean
#let _overlaps(r1, r2) = {
  return r1.x.beg < r2.x.end and r1.x.end > r2.x.beg and r1.y.beg < r2.y.end and r1.y.end > r2.y.beg
}

/// Places the beginning label
///
/// - ng (dictionary): Current node group
/// - collision (array): Collision
/// - u (length): Unit length
/// - up (boolean): If the line is going upwards
/// - elem (array): Elements to place
/// - name (content): Train's name
/// - theta (angle): How much to rotate the label
/// - m (dictionary): Train label's measures
/// - r (length): Train label's length; this value is used in a polar coordinate
/// -> array
#let _place-beg-label(ng, collision, u, up, elem, name, theta, m, r) = {
  // placing is regardless of the horizontal scale
  if up {
    let (x, y) = ng.at(0)
    y -= .25 * u
    let node_1 = (x, y)
    let node_0 = rec(p0: node_1, (theta: 180deg + theta, r: r))
    let bbox = (
      x: (beg: x - calc.cos(theta) * r, end: x + calc.sin(theta) * m.height),
      y: (beg: y - calc.sin(theta) * r - calc.cos(theta) * m.height, end: y),
    )
    // collision = collision.sorted(key: it => it.x.end)
    for i in range(MAX_LOOP) {
      let collided = collision.find(it => _overlaps(it, bbox))
      if collided == none {
        break
      }
      let dy = collided.y.beg - node_1.at(1)
      node_1.at(1) += dy
      node_0.at(1) += dy
      bbox.y.beg += dy
      bbox.y.end += dy
    }
    collision.push(bbox)
    ng.insert(0, node_0)
    ng.insert(1, node_1)
    elem.push((
      place(
        dx: node_0.at(0),
        dy: node_0.at(1) - m.height,
        rotate(theta, name, origin: bottom + left),
      )
    ))
  } else {
    let (x, y) = ng.at(0)
    y += .25 * u
    let node_1 = (x, y)
    let node_0 = rec(p0: node_1, (theta: 180deg - theta, r: r))
    let bbox = (
      x: (beg: x - calc.cos(theta) * r - calc.sin(theta) * m.height, end: x),
      y: (beg: y - calc.cos(theta) * m.height, end: y + calc.sin(theta) * r),
    )
    let y_size = bbox.y.end - bbox.y.beg
    // collision = collision.sorted(key: it => it.x.end)
    for i in range(MAX_LOOP) {
      let collided = collision.find(it => _overlaps(it, bbox))
      if collided == none {
        break
      }
      let dy = collided.y.end - node_0.at(1) + y_size
      node_1.at(1) += dy
      node_0.at(1) += dy
      bbox.y.beg += dy
      bbox.y.end += dy
    }
    collision.push(bbox)
    ng.insert(0, node_0)
    ng.insert(1, node_1)
    elem.push((
      place(
        dx: node_0.at(0),
        dy: node_0.at(1) - m.height,
        rotate(-theta, name, origin: bottom + left),
      )
    ))
  }
  (ng, elem, collision)
}

/// Places the end label
///
/// - ng (dictionary): Current node group
/// - collision (array): Collision
/// - u (length): Unit length
/// - up (boolean): If the line is going upwards
/// - elem (array): Elements to place
/// - name (content): Train's name
/// - theta (angle): How much to rotate the label
/// - m (dictionary): Train label's measures
/// - r (length): Train label's length; this value is used in a polar coordinate
/// -> array
#let _place-end-label(ng, collision, u, up, elem, name, theta, m, r) = {
  // placing is regardless of the horizontal scale
  if up {
    let (x, y) = ng.at(-1)
    y -= .25 * u
    let node_1 = (x, y)
    let node_0 = rec(p0: node_1, (theta: -theta, r: r))
    let bbox = (
      x: (beg: x - calc.sin(theta) * m.height, end: x + calc.cos(theta) * r),
      y: (beg: y - calc.sin(theta) * r - calc.cos(theta) * m.height, end: y),
    )
    // collision = collision.sorted(key: it => it.x.end)
    for i in range(MAX_LOOP) {
      let collided = collision.find(it => _overlaps(it, bbox))
      if collided == none {
        break
      }
      let dy = collided.y.beg - node_1.at(1)
      node_1.at(1) += dy
      node_0.at(1) += dy
      bbox.y.beg += dy
      bbox.y.end += dy
    }
    collision.push(bbox)
    ng.push(node_1)
    ng.push(node_0)
    elem.push((
      place(
        dx: node_1.at(0),
        dy: node_1.at(1) - m.height,
        rotate(-theta, name, origin: bottom + left),
      )
    ))
  } else {
    let (x, y) = ng.at(-1)
    y += .25 * u
    let node_1 = (x, y)
    let node_0 = rec(p0: node_1, (theta: theta, r: r))
    let bbox = (
      x: (beg: x, end: x + calc.cos(theta) * r + calc.sin(theta) * m.height),
      y: (beg: y - calc.cos(theta) * m.height, end: y + calc.sin(theta) * r),
    )
    // collision = collision.sorted(key: it => it.x.end)
    let y_size = bbox.y.end - bbox.y.beg
    // collision = collision.sorted(key: it => it.x.end)
    for i in range(MAX_LOOP) {
      let collided = collision.find(it => _overlaps(it, bbox))
      if collided == none {
        break
      }
      let dy = collided.y.end - node_0.at(1) + y_size
      node_1.at(1) += dy
      node_0.at(1) += dy
      bbox.y.beg += dy
      bbox.y.end += dy
    }
    collision.push(bbox)
    ng.push(node_1)
    ng.push(node_0)
    elem.push((
      place(
        dx: node_1.at(0),
        dy: node_1.at(1) - m.height,
        rotate(theta, name, origin: bottom + left),
      )
    ))
  }
  (ng, elem, collision)
}

/// Check if the line is running upwards on its beginning
///
/// - ng (array): Current node group
/// -> boolean
#let _check-beg-up(ng) = {
  if ng.at(1).at(1) == ng.at(0).at(1) {
    if ng.len() > 2 {
      ng.at(2).at(1) > ng.at(0).at(1)
    } else {
      false
    }
  } else if ng.at(1).at(1) > ng.at(0).at(1) {
    true
  } else {
    false
  }
}

/// Check if the line is running upwards on its end
///
/// - ng (array): Current node group
/// -> boolean
#let _check-end-up(ng) = {
  if ng.at(-1).at(1) == ng.at(-2).at(1) {
    if ng.len() > 2 {
      ng.at(-3).at(1) > ng.at(-1).at(1)
    } else {
      true
    }
  } else if ng.at(-1).at(1) > ng.at(-2).at(1) {
    false
  } else {
    true
  }
}

/// Places train labels
///
/// - ng (array): Current node group
/// - collision (array): Collision
/// - beg_time (int): Diagram beginning time
/// - end_time (int): Diagram ending time
/// - unit_length (length): Length of one unit
/// - name (content, str): Label of the train, usually the train number
/// - theta (angle): How much to rotate the label
/// - debug (boolean): Debug mode flick
/// -> array
#let _place-label(ng, collision, beg_time, end_time, unit_length, name, theta, debug: false) = {
  // check if going upwards
  let elem = ()
  let beg_flat = ng.at(0).at(0) == beg_time
  let end_flat = ng.at(-1).at(0) == end_time
  let label = box(stroke: if debug { 1pt + blue } else { none }, pad(left: 5pt, bottom: 2pt, right: 5pt)[#name])
  let m = measure(label)
  let r = m.width
  if beg_flat and end_flat {
    // both ends are flat
    (ng, elem, collision) = _place-flat-beg-label(ng, collision, unit_length, elem, label, theta, m, r)
    (ng, elem, collision) = _place-flat-end-label(ng, collision, unit_length, elem, label, theta, m, r)
  } else {
    let m2 = measure(rotate(theta, label, reflow: true))
    if beg_flat {
      // (ng, elem, collision) = _place-flat-beg-label(ng, collision, unit_length, elem, label, theta, m2, r)
    } else {
      (ng, elem, collision) = _place-beg-label(ng, collision, unit_length, _check-beg-up(ng), elem, label, theta, m, r)
    }
    if end_flat {
      // (ng, elem, collision) = _place-flat-end-label(ng, collision, unit_length, elem, label, theta, m2, r)
    } else {
      (ng, elem, collision) = _place-end-label(ng, collision, unit_length, _check-end-up(ng), elem, label, theta, m, r)
    }
  }
  (ng, elem, collision)
}

/// Make trains
///
/// - collision (array): Global collision array
/// - trains (dictionary): Trains to process
/// - stations (dictionary): Stations to process
/// - beg_time (int): Diagram beginning time
/// - end_time (int): Diagram end time
/// - track_scale (int, float): How much to scale the track
/// - unit_length (length): Length of one unit
/// - horizontal_scale (int, float): How much to scale the horizontal axis
/// - train_coloring (str, none, auto): How to color the trains
/// - show_label (boolean): Whether to show train labels
/// - debug (boolean): Debug mode flick
/// -> array
#let make-trains(
  collision,
  trains,
  stations,
  beg_time,
  end_time,
  track_scale,
  unit_length,
  horizontal_scale,
  train_coloring,
  show_label,
  debug: false,
) = {
  let elem = ()
  for train in trains {
    let seed = array(bytes(train.at(0))).sum()
    let node_group = _process-train-nodes(
      train,
      stations,
      beg_time,
      end_time,
      track_scale,
    )
    for ng in node_group {
      // walks at most two nodes
      ng = ng.map(it => (to-x(it.at(0) - beg_time) * unit_length * horizontal_scale, it.at(1) * unit_length))
      let e = ()
      if show_label {
        (ng, e, collision) = _place-label(
          ng,
          collision,
          beg_time,
          end_time,
          unit_length,
          train.at(1).at("name", default: train.at(0)),
          15deg,
          debug: debug,
        )
        for e in e {
          elem.push(e)
        }
      }
      let ng0 = ng.at(0)
      if debug {
        for (i, n) in ng.enumerate() {
          elem.push(
            place(
              dx: n.at(0) - 2pt,
              dy: n.at(1) - 2pt,
              circle(radius: 2pt, stroke: none, fill: black, text(size: .5em)[#i]),
            ),
          )
        }
      }
      ng = ng.map(it => curve.line(it))
      if train_coloring != auto and train_coloring != none and (train.at(1).at("stroke", default: none)) == array {
        for s in train.at(1).stroke {
          elem.push(
            place(
              curve(
                stroke: s,
                curve.move(ng0),
                ..ng,
              ),
            ),
          )
        }
      } else {
        let s = train.at(1).at("stroke", default: red)
        if train_coloring == auto {
          let rng = suiji.gen-rng-f(seed)
          s = 1pt + color.oklch(70%, 40%, 1deg * suiji.integers-f(rng, low: 0, high: 360).at(1))
        } else if train_coloring == none {
          s = 1pt + gray.darken(30%)
        }
        elem.push(
          place(
            curve(
              stroke: s,
              curve.move(ng0),
              ..ng,
            ),
          ),
        )
      }
    }
  }
  (elem, collision)
}
