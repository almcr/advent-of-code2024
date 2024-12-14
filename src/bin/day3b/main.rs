use regex::Regex;

fn main() {
  let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
  let mut enabled = true;

  println!(
    "{}",
    re.captures_iter(include_str!("input"))
      .filter(|c| {
        if c.get(0).unwrap().as_str() == "do()" {
          enabled = true;
          return false;
        } else if c.get(0).unwrap().as_str() == "don't()" {
          enabled = false;
        }
        enabled
      })
      .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap()
        * c.get(2).unwrap().as_str().parse::<i32>().unwrap())
      .sum::<i32>()
  );
}
