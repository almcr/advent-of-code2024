fn main() {
  let map = include_str!("input").lines().collect::<Vec<_>>();
  let mut visited = vec![vec![false; map[0].len()]; map.len()];
  let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
  let mut d = 0;
  let mut pos = map
    .iter()
    .enumerate()
    .filter_map(|(i, l)| l.find("^").map(|o| (i, o)))
    .next();
  let mut n = 1;
  while let Some((x, y)) = pos {
    visited[x][y] = true;
    let (xx, yy) = (
      (x as i32 + directions[d].0) as usize,
      (y as i32 + directions[d].1) as usize,
    );

    match map.get(xx).and_then(|e| e.chars().nth(yy)) {
      Some('#') => d = (d + 1) % 4,
      Some(_) => {
        pos = Some((xx, yy));
        if !visited[xx][yy] {
          n += 1;
        }
      }
      None => break,
    }
  }
  println!("{}", n);
}
