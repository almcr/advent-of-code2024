use std::collections::HashSet;

type Set = HashSet<(usize, usize)>;

fn find_region(map: &[&[u8]], x: usize, y: usize, seen: &mut Set, shape: &mut Set) {
  for (xx, yy) in [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)] {
    if seen.contains(&(xx, yy)) {
      continue;
    }
    if map.get(xx).and_then(|v| v.get(yy)) == Some(&map[x][y]) {
      shape.insert((xx, yy));
      seen.insert((xx, yy));
      find_region(map, xx, yy, seen, shape);
    }
  }
}

fn num_sides(region: &Set) -> usize {
  let mut sides_end = HashSet::new();
  for &(x, y) in region {
    for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
      if region.contains(&(x + dx as usize, y + dy as usize)) {
        continue;
      }
      let (mut x, mut y) = (x, y);
      while region.contains(&(x + dy as usize, y + dx as usize))
        && !region.contains(&(x + dx as usize, y + dy as usize))
      {
        x += dy as usize;
        y += dx as usize;
      }
      sides_end.insert((x, y, dx, dy));
    }
  }
  sides_end.len()
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
      let p = num_sides(&reg);
      s += p * reg.len();
    }
  }
  println!("{}", s);
}
