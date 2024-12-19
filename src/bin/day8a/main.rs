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
  antenas.values().for_each(|v| {
    v.iter().tuple_combinations().for_each(|(t1, t2)| {
      [(t1, t2), (t2, t1)].iter().for_each(|(a, b)| {
        let anode = (a.0 + (a.0 - b.0), a.1 + (a.1 - b.1));
        if (0..map.len() as isize).contains(&anode.0)
          && (0..map[0].len() as isize).contains(&anode.1)
        {
          antinodes.insert(anode);
        }
      });
    });
  });
  println!("{}", antinodes.len());
}
