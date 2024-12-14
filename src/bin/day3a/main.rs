use regex::Regex;

fn main() {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  println!(
    "{}",
    re.captures_iter(include_str!("input"))
      .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap()
        * c.get(2).unwrap().as_str().parse::<i32>().unwrap())
      .sum::<i32>()
  )
}
