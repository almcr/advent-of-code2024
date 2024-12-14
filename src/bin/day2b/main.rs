use itertools::Itertools;

fn safe<'a>(nums: impl Iterator<Item = &'a i32>) -> bool {
  nums
    .tuple_windows()
    .try_fold(0, |is_incr, (a, b)| {
      if is_incr >= 0 && (1..=3).contains(&(b - a)) {
        Ok(1)
      } else if is_incr <= 0 && (1..=3).contains(&(a - b)) {
        Ok(-1)
      } else {
        Err(())
      }
    })
    .is_ok()
}

fn main() {
  let num_safe = include_str!("input")
    .lines()
    .filter(|line| {
      let nums = line
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

      (0..nums.len()).any(|i| safe(nums[0..i].iter().chain(&nums[i + 1..])))
    })
    .count();

  println!("{num_safe}");
}
