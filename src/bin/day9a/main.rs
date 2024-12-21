fn main() {
  let map = include_bytes!("input");
  let mut blocks = Vec::new();
  let mut fd = 0;
  for (i, b) in map.iter().enumerate() {
    if i % 2 == 0 {
      blocks.extend((0..b - b'0').map(|_| fd));
      fd += 1;
    } else {
      blocks.extend((0..b - b'0').map(|_| -1))
    }
  }
  let mut j = blocks.len() - 1;
  let mut i = blocks.iter().position(|&b| b == -1).unwrap();
  while i < j {
    if blocks[j] == -1 {
      j -= 1;
      continue;
    }
    blocks[i] = blocks[j];
    j -= 1;
    i = i + blocks[i + 1..].iter().position(|&b| b == -1).unwrap() + 1;
  }
  let s: usize = blocks[..j + 1]
    .iter()
    .enumerate()
    .map(|(ii, &b)| if b == -1 { 0 } else { ii * b as usize })
    .sum();
  println!("{s}");
}
