fn main() {
  let mut stones = include_str!("input")
    .split(" ")
    .map(|x| x.parse::<i64>().unwrap())
    .collect::<Vec<_>>();

  for _ in 0..25 {
    let mut i = 0;
    while i < stones.len() {
      let s = stones[i];
      match s {
        0 => stones[i] = 1,
        _ if s.ilog10() % 2 == 1 => {
          let div = 10_i64.pow((s.ilog10() + 1) / 2);
          stones[i] /= div;
          stones.insert(i + 1, s % div);
          i += 1
        }
        _ => stones[i] *= 2024,
      }
      i += 1;
    }
  }
  println!("{}", stones.len());
}
