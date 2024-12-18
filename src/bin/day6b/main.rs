use std::collections::HashSet;

use itertools::Itertools;

fn main() {
  let mut map = include_str!("input")
    .lines()
    .map(|l| l.as_bytes().to_vec())
    .collect::<Vec<_>>();
  let mut visited = HashSet::new();
  let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

  let start = (0..map.len())
    .cartesian_product(0..map[0].len())
    .find(|&(i, j)| map[i][j] == b'^');

  let simulate = |mut pos: Option<(usize, usize)>,
                  mut d: usize,
                  map: &Vec<Vec<_>>,
                  mut visited: Option<&mut HashSet<(usize, usize)>>|
   -> bool {
    let mut seen = vec![vec![[false; 4]; map[0].len()]; map.len()];
    while let Some((x, y)) = pos {
      if seen[x][y][d] {
        return true;
      }
      seen[x][y][d] = true;
      if let Some(v) = visited.as_deref_mut() {
        v.insert((x, y));
      }
      let (xx, yy) = (
        (x as i32 + directions[d].0) as usize,
        (y as i32 + directions[d].1) as usize,
      );

      match map.get(xx).and_then(|e| e.get(yy)) {
        Some(b'#') => d = (d + 1) % 4,
        Some(_) => {
          pos = Some((xx, yy));
        }
        None => break,
      }
    }
    false
  };

  // first round to get initial path
  simulate(start, 0, &map, Some(&mut visited));

  // then simulate an obstacle in each position of the path
  println!(
    "{}",
    visited
      .iter()
      .filter(|&&(i, j)| {
        map[i][j] = b'#';
        let b = simulate(start, 0, &map, None);
        map[i][j] = b'.';
        b
      })
      .count()
  );
}
