use itertools::Itertools;

fn main() {
  let mut robots = include_str!("input")
    .split(|c: char| !c.is_ascii_digit() && c != '-')
    .filter(|s| !s.is_empty())
    .map(|c| c.parse::<i64>().unwrap())
    .tuples()
    .collect::<Vec<_>>();

  for i in 0.. {
    for (x, y, dx, dy) in &mut robots {
      *x = (*x + *dx).rem_euclid(101);
      *y = (*y + *dy).rem_euclid(103);
    }
    if robots.iter().map(|&(x, y, _, _)| (x, y)).all_unique() {
      println!("{}", i + 1);
      break;
    }
  }
}
