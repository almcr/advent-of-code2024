#![allow(arithmetic_overflow)]
use z3::ast::BV;

fn main() {
  let program = [2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 4, 5, 5, 3, 0];

  let opt = z3::Optimize::new();

  let s = BV::new_const("s", 64);

  let (mut a, mut b, mut c) = (s.clone(), BV::from_i64(0, 64), BV::from_i64(0, 64));

  for x in program {
    b = &a & &BV::from_i64(7, 64);
    b ^= BV::from_i64(1, 64);
    c = a.bvlshr(&b);
    b ^= BV::from_i64(5, 64);
    a = a.bvlshr(BV::from_i64(3, 64));
    b ^= c;
    opt.assert(&(b & BV::from_i64(7, 64)).eq(BV::from_i64(x, 64)));
  }
  opt.assert(&a.eq(BV::from_i64(0, 64)));

  opt.minimize(&s);
  assert_eq!(opt.check(&[]), z3::SatResult::Sat);

  println!(
    "{}",
    opt
      .get_model()
      .unwrap()
      .eval(&s, true)
      .unwrap()
      .as_i64()
      .unwrap()
  );
}
