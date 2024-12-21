fn main() {
  let map = include_bytes!("input");
  let mut blocks = Vec::new();
  let mut fd = 0;
  for (i, b) in map.iter().enumerate() {
    if i % 2 == 0 {
      blocks.push((b - b'0', fd));
      fd += 1;
    } else {
      blocks.push((b - b'0', -1));
    }
  }
  let mut j = blocks.len() - 1;
  while j > 0 {
    let (size_j, fd) = blocks[j];
    if fd == -1 {
      j -= 1;
      continue;
    }
    if let Some(i) = blocks[..j]
      .iter()
      .position(|&(size, id)| id == -1 && size >= size_j)
    {
      let (size, _) = blocks[i];
      blocks[i] = blocks[j];
      blocks[j] = (size_j, -1);
      if size > size_j {
        // this insert is expensive
        blocks.insert(i + 1, (size - size_j, -1));
      }
    }
    j -= 1;
  }
  let s = blocks
    .iter()
    .flat_map(|&(s, id)| (0..s).map(move |_| id as i64))
    .enumerate()
    .map(|(i, id)| if id == -1 { 0 } else { i * id as usize })
    .sum::<usize>();
  println!("{s}");
}
