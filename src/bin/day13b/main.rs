use itertools::Itertools;

fn main() {
  let solve = |a1: i64, a2: i64, b1: i64, b2: i64, c1: i64, c2: i64| {
    let det = a1 * b2 - a2 * b1;
    let dx = c1 * b2 - c2 * b1;
    let dy = a1 * c2 - a2 * c1;
    let (a, b) = (dx / det, dy / det);
    if (a * a1 + b * b1, a * a2 + b * b2) != (c1, c2) {
      return 0;
    }
    3 * a + b
  };
  let pad = 10000000000000;
  let s = include_str!("input")
    .split(|c: char| !c.is_ascii_digit())
    .filter(|s| !s.is_empty())
    .map(|s| s.parse().unwrap())
    .tuples()
    .fold(0, |acc, (a1, a2, b1, b2, c1, c2)| {
      acc + solve(a1, a2, b1, b2, c1 + pad, c2 + pad)
    });
  println!("{}", s);
}
