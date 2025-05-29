#![allow(arithmetic_overflow)]
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
  let mut map: Vec<_> = map.lines().map(|l| l.as_bytes().to_vec()).collect();
  let (mut x, mut y) = (0..map.len())
    .cartesian_product(0..map[0].len())
    .find(|&(i, j)| map[i][j] == b'@')
    .unwrap();
  let moves: Vec<_> = moves.lines().flat_map(|l| l.as_bytes()).collect();

  for m in moves {
    let (dx, dy) = direction(*m);
    // how far can we push
    let (mut xx, mut yy) = (x + dx as usize, y + dy as usize);
    while map[xx][yy] == b'O' {
      (xx, yy) = (xx + dx as usize, yy + dy as usize);
    }
    if map[xx][yy] == b'#' {
      // we can't push anything
      continue;
    }
    while (xx, yy) != (x, y) {
      map[xx][yy] = map[xx - dx as usize][yy - dy as usize];
      (xx, yy) = (xx - dx as usize, yy - dy as usize);
    }
    map[x][y] = b'.';
    (x, y) = (x + dx as usize, y + dy as usize);
  }
  let s = (0..map.len())
    .cartesian_product(0..map[0].len())
    .filter(|&(x, y)| map[x][y] == b'O')
    .map(|(x, y)| x * 100 + y)
    .sum::<usize>();
  println!("{s}");
}
