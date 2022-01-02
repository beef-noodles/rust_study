pub fn run() {
  println!("Hello, world!");
  if !true {
    println!("true");
  } else if true {
    println!("false");
  }

  let v = if true { 2 } else { 4 };
  println!("{}", v);

  let mut i = 0;
  loop {
    if i == 5 {
      i += 1;
      continue;
    }
    if i >= 10 {
      break;
    }
    i = i + 1;
  }
  println!("end i = {}", i);

  let vv = vec![1, 2, 3, 4];
  for element in vv {
    println!("{}", element);
  }

  struct Circle {
    x: f64,
    y: f64,
    radius: f64,
  }

  impl Circle {
    fn area(&self) -> f64 {
      std::f64::consts::PI * (self.radius * self.radius)
    }
  }
  let circle = Circle {
    x: 0.0,
    y: 0.0,
    radius: 10.0,
  };
  println!("location [{0},{1}]", circle.x, circle.y);
  println!("{}", circle.area());

  impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
      Circle {
        x: x,
        y: y,
        radius: radius,
      }
    }
  }
  let circle1 = Circle::new(0.0, 0.0, 10.0);
  println!("{} {}, {}", circle1.x, circle1.y, circle1.area());

  // let mut temp = Vec::new();
  // temp.push(1);
  // temp.push(2);
  // let v = temp;
  // let v: Vec<i32> = vec![1, 2, 3, 4];
  // let v: Vec<i32> = vec![1; 10];
  let v: Vec<_> = (0..10).collect();
  println!("{:?}", v);
  println!("{:?}", v.get(9));
  assert_eq!(v.get(9), Some(&9));
  assert_eq!(v.get(10), None);
  println!("=======================");
  let testStr = "tests";
  // fn test() {
  //   testStr = "123";
  //   println!("{}", testStr);
  // }
  println!("{}", testStr);

  let x = "哎呦我去".to_string();
  for i in x.chars() {
    println!("{}", i)
  }
  println!("{:?}", x.chars().nth(2));
  let a = String::from("Hello world guzhongren");
  let b = a.clone();
  println!("{}", b)
}
