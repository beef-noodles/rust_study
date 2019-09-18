struct Val {
  val: f64,
}

struct GenVal<T> {
  gen_val: T,
}

impl Val {
  fn value(&self) -> &f64 {
    &self.val
  }
  fn customs(&self)-> f64 {
    let a = 2f64;
    self.val + a
  }
}

impl <T>GenVal<T> {
  fn value(&self) -> &T {
    &self.gen_val
  }
}

pub fn run() {
  let x = Val { val: 64.0};
  let y = GenVal { gen_val: 4i32 };
  println!("{}, {}, {}", x.customs(), x.value(), y.value());
  // 66, 64, 4
}
