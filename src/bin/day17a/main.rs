#![allow(arithmetic_overflow)]
use itertools::Itertools;

fn main() {
  let mut input = include_str!("input")
    .split(|c: char| !c.is_ascii_digit())
    .filter(|c| !c.is_empty())
    .map(|c| c.parse::<i64>().unwrap());
  let (mut a, mut b, mut c) = input.next_tuple().unwrap();
  let program = input.collect::<Vec<_>>();

  let operand = |o, a, b, c| match o {
    0..=3 => o,
    4 => a,
    5 => b,
    6 => c,
    _ => unreachable!(),
  };

  let mut output = Vec::new();
  while a != 0 {
    for (&opcode, &o) in program.iter().tuples() {
      let operand = operand(o, a, b, c);
      match opcode {
        0 => a /= 1 << operand,      // adv
        1 => b ^= o,                 // bxl
        2 => b = operand % 8,        // bst
        3 => continue,               // jnz
        4 => b ^= c,                 // bxc
        6 => b = a / (1 << operand), // bdv
        7 => c = a / (1 << operand), // cdv
        5 => output.push(operand % 8),
        _ => unreachable!(),
      }
    }
  }
  println!("{}", output.iter().map(|c| c.to_string()).join(","))
}
