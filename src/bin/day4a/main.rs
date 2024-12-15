fn main() {
  let map: Vec<_> = include_str!("input").lines().collect();
  let mut word = [0; 4];
  let directions = [
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (1, 1), (2, 2), (3, 3)],
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(0, 0), (0, -1), (0, -2), (0, -3)],
    [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
    [(0, 0), (-1, 0), (-2, 0), (-3, 0)],
    [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
    [(0, 0), (1, -1), (2, -2), (3, -3)],
  ];
  println!(
    "{}",
    (0..map.len() as isize)
      .flat_map(|x| (0..map[0].len() as isize).map(move |y| (x, y)))
      .filter(|(x, y)| {
        map
          .get(*x as usize)
          .and_then(|row| row.chars().nth(*y as usize))
          == Some('X')
      })
      .map(|(x, y)| {
        directions
          .iter()
          .filter(|coords| {
            let mut iter = coords.iter().map(|(xx, yy)| {
              map
                .get((x + xx) as usize)
                .and_then(|row| row.chars().nth((y + yy) as usize))
                .unwrap_or_default()
            });
            word.fill_with(|| iter.next().unwrap_or_default() as u8);
            &word == b"XMAS" || &word == b"SAMX"
          })
          .count()
      })
      .sum::<usize>()
  );
}
