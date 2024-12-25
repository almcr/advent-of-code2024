use itertools::Itertools;

fn main() {
  let mut robots = include_str!("input")
    .split(|c: char| !c.is_ascii_digit() && c != '-')
    .filter(|s| !s.is_empty())
    .map(|c| c.parse::<i64>().unwrap())
    .tuples()
    .collect::<Vec<_>>();

  for _ in 0..100 {
    for (x, y, dx, dy) in &mut robots {
      *x = (*x + *dx).rem_euclid(101);
      *y = (*y + *dy).rem_euclid(103);
    }
  }
  let s = robots
    .into_iter()
    .filter(|&(x, y, _, _)| x != (101 / 2) && y != (103 / 2))
    .fold([0; 4], |mut quadt, r| {
      quadt[2 * (r.0 > (101 / 2)) as usize + (r.1 > (103 / 2)) as usize] += 1;
      quadt
    })
    .iter()
    .product::<usize>();
  println!("{:?}", s);
}
