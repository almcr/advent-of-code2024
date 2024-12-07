use itertools::Itertools;

pub fn main() {
  let num_safe = include_str!("input")
    .lines()
    .filter(|line| {
      line
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .tuple_windows()
        .all(|(a, b, c)| {
          let diff_ab = a.abs_diff(b);
          let diff_bc = b.abs_diff(c);
          ((a < b && b < c) || (a > b && b > c))
            && (1..=3).contains(&diff_ab)
            && (1..=3).contains(&diff_bc)
        })
    })
    .count();

  println!("{num_safe}");
}
