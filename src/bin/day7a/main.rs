fn is_valid(target: i64, n: i64, values: &[i64]) -> bool {
  if values.is_empty() || n > target {
    return n == target;
  }
  is_valid(target, n * values[0], &values[1..]) || is_valid(target, n + values[0], &values[1..])
}

fn main() {
  let m = include_str!("input")
    .lines()
    .map(|l| {
      let (total, values) = l.split_once(": ").unwrap();
      let values = values
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();
      (total.parse().unwrap(), values)
    })
    .collect::<Vec<_>>();
  println!(
    "{}",
    m.iter()
      .filter_map(|(total, values)| is_valid(*total, values[0], &values[1..]).then_some(total))
      .sum::<i64>()
  )
}
