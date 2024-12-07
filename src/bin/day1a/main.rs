pub fn main() {
  let mut a = Vec::new();
  let mut b = Vec::new();
  include_str!("input").lines().for_each(|line| {
    let mut numbers = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
    let left = numbers.next().unwrap();
    let right = numbers.next().unwrap();
    a.push(left);
    b.push(right);
  });

  a.sort();
  b.sort();

  let s = a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum::<u32>();

  println!("{s}")
}
