use itertools::Itertools;

fn main() {
  let map = include_str!("input")
    .lines()
    .map(|l| l.as_bytes().to_vec())
    .collect::<Vec<_>>();
  let mut visited = vec![vec![false; map[0].len()]; map.len()];
  let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
  let mut d = 0;
  let mut pos = (0..map.len())
    .cartesian_product(0..map[0].len())
    .find(|&(i, j)| map[i][j] == b'^');
  let mut n = 1;
  while let Some((x, y)) = pos {
    visited[x][y] = true;
    let (xx, yy) = (
      (x as i32 + directions[d].0) as usize,
      (y as i32 + directions[d].1) as usize,
    );

    match map.get(xx).and_then(|e| e.get(yy)) {
      Some(b'#') => d = (d + 1) % 4,
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
