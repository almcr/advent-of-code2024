use std::{cmp::Ordering, collections::HashMap};

fn main() {
  let mut data_iter = include_str!("input").split("\n\n");

  let orders: HashMap<i32, Vec<_>> =
    data_iter
      .next()
      .unwrap()
      .split("\n")
      .fold(HashMap::new(), |mut orders, order| {
        let mut split = order.split("|");
        orders
          .entry(split.next().unwrap().parse::<i32>().unwrap())
          .or_default()
          .push(split.next().unwrap().parse::<i32>().unwrap());
        orders
      });

  println!(
    "{}",
    data_iter
      .next()
      .unwrap()
      .split("\n")
      .map(|pages| {
        pages
          .split(",")
          .map(|p| p.parse::<i32>().unwrap())
          .collect::<Vec<_>>()
      })
      .filter(|pages| {
        for (i, page) in pages.iter().enumerate() {
          if let Some(orders) = orders.get(page) {
            if pages[0..i].iter().any(|&page| orders.contains(&page)) {
              return true;
            }
          }
        }
        false
      })
      .map(|mut pages| {
        pages.sort_unstable_by(|a, b| {
          if orders.get(a).is_some_and(|orders| orders.contains(b)) {
            Ordering::Less
          } else {
            Ordering::Greater
          }
        });
        pages[pages.len() / 2]
      })
      .sum::<i32>()
  );
}
