pub fn main() {
  let mut c = [0; 100_000];
  let mut a = Vec::new();
  include_str!("input").lines().for_each(|line| {
    let mut numbers = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
    let left = numbers.next().unwrap();
    let right = numbers.next().unwrap();
    a.push(left);
    c[right as usize] += 1;
  });

  let s: i32 = a.iter().map(|n| n * c[*n as usize]).sum();
  println!("{s}")
}
