#![allow(arithmetic_overflow)]
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
  let map: Vec<_> = include_str!("input")
    .lines()
    .map(|l| l.as_bytes())
    .collect();

  let mut visited = HashMap::new();

  // search for start and end positions
  let (mut start, mut end) = ((0, 0), (0, 0));
  for i in 0..map.len() {
    for j in 0..map[0].len() {
      match map[i][j] {
        b'S' => start = (i, j),
        b'E' => end = (i, j),
        _ => continue,
      }
    }
  }

  let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

  // (score, direction index, (i, j coords))
  let mut q = BinaryHeap::from([(0, 0, start)]);
  let mut lowest_score = i64::MAX;
  while let Some((score, d, (x, y))) = q.pop() {
    let score = -score;
    if (x, y) == end {
      if score > lowest_score {
        break;
      }
      lowest_score = score;
    }

    for di in 0..4 {
      let (dx, dy) = directions[di];
      let (xx, yy) = (x + dx as usize, y + dy as usize);
      if map[xx][yy] == b'#' {
        continue;
      }
      let s = score + if d == di { 1 } else { 1001 };
      // adding the direction here in the key to be able to backtrack
      let score = visited.get(&(xx, yy, di)).copied().unwrap_or(i64::MAX);
      if s <= score {
        visited.insert((xx, yy, di), s);
        q.push((-s, di, (xx, yy)));
      }
    }
  }

  let mut q = VecDeque::from([(end, 2, lowest_score)]);
  let mut seen = HashSet::new();

  while let Some(((x, y), d, s)) = q.pop_front() {
    seen.insert((x, y));
    for dd in 0..4 {
      let ns = s - if dd == d { 1 } else { 1001 };
      let (dx, dy) = directions[d];
      let (xx, yy) = (x - dx as usize, y - dy as usize);
      let ss = visited.get(&(xx, yy, dd)).copied().unwrap_or(i64::MAX);
      if ss == ns {
        q.push_back(((xx, yy), dd, ns));
      }
    }
  }

  println!("{}", seen.len() + 1);
}
