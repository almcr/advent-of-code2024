use std::collections::HashSet;

type Set = HashSet<(usize, usize)>;

fn find_region(map: &[&[u8]], x: usize, y: usize, seen: &mut Set, shape: &mut Set) {
  for (xx, yy) in [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)] {
    if seen.contains(&(xx, yy)) {
      continue;
    }
    if *map.get(xx).and_then(|v| v.get(yy)).unwrap_or(&0) == map[x][y] {
      shape.insert((xx, yy));
      seen.insert((xx, yy));
      find_region(map, xx, yy, seen, shape);
    }
  }
}

fn perimeter(m: &[&[u8]], region: &Set) -> usize {
  region
    .iter()
    .map(|&(x, y)| {
      4 - [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)]
        .iter()
        .filter(|(xx, yy)| *m.get(*xx).and_then(|v| v.get(*yy)).unwrap_or(&0) == m[x][y])
        .count()
    })
    .sum()
}

fn main() {
  let map = include_str!("input")
    .lines()
    .map(|l| l.as_bytes())
    .collect::<Vec<_>>();
  let mut seen = HashSet::new();
  let mut s = 0;
  for i in 0..map.len() {
    for j in 0..map[0].len() {
      if seen.contains(&(i, j)) {
        continue;
      }
      let mut reg = HashSet::from([(i, j)]);
      find_region(&map, i, j, &mut seen, &mut reg);
      let p = perimeter(&map, &reg);
      s += p * reg.len();
    }
  }
  println!("{}", s);
}
