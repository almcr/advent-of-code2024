#![allow(arithmetic_overflow)]
use std::collections::{BinaryHeap, HashMap};

fn main() {
  let map: Vec<_> = include_str!("input")
    .lines()
    .map(|l| l.as_bytes())
    .collect();
  let mut distances = HashMap::new();

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
      let s = score + if directions[d] == (dx, dy) { 1 } else { 1001 };
      let dist = distances.get(&(xx, yy)).copied().unwrap_or(i64::MAX);
      if s < dist {
        distances.insert((xx, yy), s);
        q.push((-s, di, (xx, yy)));
      }
    }
  }
  println!("{lowest_score}");
}
