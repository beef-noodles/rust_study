use std::ops::Add;

pub fn run() {
  // fn add<T: Add<T, Output=T>>(a: T, b: T) -> T {
  //   a + b
  // }

  // println!("{}", add(100, 1));


  #[derive(Debug)]
  struct Point<T> {
    x: T,
    y: T,
  }
  impl <T: Add<T, Output=T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Point<T> {
      Point{
        x: self.x + p.x,
        y: self.y + p.y,
      }
    }
  }
  
  fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
  }
  let  point1 = Point{
    x: 1,
    y: 1
  };
  let point2 = Point{
    x: 2,
    y: 2,
  };

  println!("{:?}", add(point1, point2));
}