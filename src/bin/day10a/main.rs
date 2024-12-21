use std::collections::HashSet;

fn path_find(map: &[&[u8]], x: usize, y: usize, seen: &mut HashSet<(usize, usize)>) {
  let h = map.get(x).and_then(|v| v.get(y)).unwrap_or(&0);
  if h == &b'9' {
    seen.insert((x, y));
    return;
  }
  for (xx, yy) in [
    (x.wrapping_sub(1), y),
    (x, y + 1),
    (x + 1, y),
    (x, y.wrapping_sub(1)),
  ] {
    if *map.get(xx).and_then(|v| v.get(yy)).unwrap_or(&0) == *h + 1 {
      path_find(map, xx, yy, seen);
    }
  }
}

fn main() {
  let map: Vec<_> = include_str!("input")
    .lines()
    .map(|l| l.as_bytes())
    .collect();
  let mut seen = HashSet::new();
  let mut s = 0;
  for i in 0..map.len() {
    for j in 0..map[0].len() {
      if map[i][j] == b'0' {
        path_find(&map, i, j, &mut seen);
        s += seen.len();
        seen.clear();
      }
    }
  }
  println!("{s}");
}
