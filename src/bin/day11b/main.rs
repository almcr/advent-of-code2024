use std::collections::HashMap;

fn main() {
  let mut stones = include_str!("input")
    .split(" ")
    .map(|x| (x.parse::<i64>().unwrap(), 1))
    .collect::<HashMap<_, _>>();

  for _ in 0..75 {
    let mut new_stones = HashMap::<i64, usize>::new();
    for (s, n) in stones.iter() {
      match s {
        0 => *new_stones.entry(1).or_default() += n,
        _ if s.ilog10() % 2 == 1 => {
          let div = 10_i64.pow((s.ilog10() + 1) / 2);
          *new_stones.entry(s / div).or_default() += n;
          *new_stones.entry(s % div).or_default() += n;
        }
        _ => *new_stones.entry(s * 2024).or_default() += n,
      }
    }
    stones = new_stones;
  }
  println!("{}", stones.values().sum::<usize>());
}
