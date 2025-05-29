#![allow(arithmetic_overflow)]
use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn direction(d: u8) -> (i32, i32) {
  match d {
    b'<' => (0, -1),
    b'v' => (1, 0),
    b'>' => (0, 1),
    b'^' => (-1, 0),
    _ => unreachable!(),
  }
}

fn main() {
  let (map, moves) = include_str!("input").split_once("\n\n").unwrap();
  let mut map: Vec<_> = map
    .lines()
    .map(|l| {
      l.bytes()
        .flat_map(|b| match b {
          b'O' => b"[]",
          b'#' => b"##",
          b'.' => b"..",
          b'@' => b"@.",
          _ => unreachable!(),
        })
        .copied()
        .collect::<Vec<_>>()
    })
    .collect();

  let (mut x, mut y) = (0..map.len())
    .cartesian_product(0..map[0].len())
    .find(|&(i, j)| map[i][j] == b'@')
    .unwrap();
  let moves: Vec<_> = moves.lines().flat_map(|l| l.as_bytes()).collect();

  let mut deq = VecDeque::new();
  let mut seen = HashSet::new();

  for m in moves {
    deq.push_back((x, y));
    let (dx, dy) = direction(*m);
    let mut no_shift = false;
    // mark boxes to be displaced if possible
    while let Some((r, c)) = deq.pop_front() {
      if !seen.insert((r, c)) {
        continue;
      }
      let (xx, yy) = (r + dx as usize, c + dy as usize);
      match map[xx][yy] {
        b']' => deq.extend([(xx, yy), (xx, yy - 1)]),
        b'[' => deq.extend([(xx, yy), (xx, yy + 1)]),
        b'#' => {
          no_shift = true;
          break;
        }
        _ => continue,
      }
    }
    if no_shift {
      deq.clear();
      seen.clear();
      continue;
    }
    // we need to first order the boxes by distance from
    // robot position to be able to shift them correctly
    for &(r, c) in seen
      .iter()
      .sorted_by_key(|&&(rr, cc)| (y.abs_diff(cc), x.abs_diff(rr)))
      .rev()
    {
      let (r2, c2) = (r + dx as usize, c + dy as usize);
      map[r2][c2] = map[r][c];
      map[r][c] = b'.';
    }
    (x, y) = (x + dx as usize, y + dy as usize);
    deq.clear();
    seen.clear();
  }

  let s = (0..map.len())
    .cartesian_product(0..map[0].len())
    .filter(|&(x, y)| map[x][y] == b'[')
    .map(|(x, y)| x * 100 + y)
    .sum::<usize>();
  println!("{s}");
}
