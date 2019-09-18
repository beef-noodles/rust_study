// use std::fmt::Debug;
trait HasArea {
  fn area(&self) -> f64;
}

pub fn run() {
  struct Circle {
    x: f64,
    y: f64,
    radius: f64,
  }

  impl HasArea for Circle {
    fn area(&self) -> f64 {
      std::f64::consts::PI * (self.radius * self.radius)
    }
  }

  let c = Circle{
    x: 0.0,
    y: 0.0,
    radius: 10.0,
  };
  println!("location: [{}, {}]; Area: {}", c.x, c.y, c.area());


  // fn foo<K: Clone, T: Debug + Clone> (s: T, q: K) {
  //   println!("{:?}", s)
  // }

  // fn foo1<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
  //   x.clone();
  //   y.clone();
  //   println!("{:?}",y)
  // }

  trait Foo {
    fn foo(&self);
  }

  trait FooBa: Foo {
    fn foo_bar(&self);
  }
  #[derive(Debug)]
  struct Baz;

  impl Foo for Baz {
    fn foo(&self) {
      println!("foo");
    }
  }
  impl FooBa for Baz {
    fn foo_bar(&self) {
      println!("Foobar");
    }
  }
  let baz = Baz{};
  baz.foo();
  // println!("{}",baz.fooBar());

  #[derive(Debug)]
  struct Food{
    a: i32,
    b: bool,
  }
  impl Copy for Food {}
  impl Clone for Food {
    fn clone (&self) -> Food {
      Food {
        a: self.a,
        b: self.b,
      }
    }
  }
  let x = Food {a: 12, b: true};
  let mut y = x;
  y.b = false;
  println!("{:?}", x);
  println!("{:?}", y);

  let mut aa = String::from("abc");
  let bb ;
  {
    let mut some_clousure = |c: char| aa.push(c);
    bb = some_clousure('e');
  }
  println!("{:?}", aa);
  println!("bb:{:?}", bb);
}