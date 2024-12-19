use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
  let map = include_str!("input")
    .lines()
    .map(|l| l.as_bytes())
    .collect::<Vec<_>>();
  let mut antenas = HashMap::<_, Vec<_>>::new();
  for i in 0..map.len() {
    for j in 0..map[0].len() {
      if map[i][j] != b'.' {
        antenas
          .entry(map[i][j])
          .or_default()
          .push((i as isize, j as isize));
      }
    }
  }
  let mut antinodes = HashSet::new();
  for v in antenas.values() {
    for (t1, t2) in v.iter().tuple_combinations() {
      for (&a, &b) in [(t1, t2), (t2, t1)] {
        let ab = (a.0 - b.0, a.1 - b.1);
        let mut anode = (a.0 + ab.0, a.1 + ab.1);
        antinodes.insert(a);
        while (0..map.len() as isize).contains(&anode.0)
          && (0..map[0].len() as isize).contains(&anode.1)
        {
          antinodes.insert(anode);
          anode = (anode.0 + ab.0, anode.1 + ab.1);
        }
      }
    }
  }
  println!("{}", antinodes.len());
}
