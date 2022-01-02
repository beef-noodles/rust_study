
pub fn run() {
  let x: Vec<i32> = vec!(1i32, 2,4);
  let y = &x;
  println!("{:?}, {:?}", x, y);


  let mut a:i32 = 100;
  {
    let b: &mut i32 = &mut a;
    *b += 2;
    println!("{}", b);
  }
  println!("{:?}", a);

  // lifetime
  fn foo <'a>(x: &'a str, y: &'a str) -> &'a str {
    if true {
      x
    } else {
      y
    }
  }
  println!("{:?}",foo("test1", "test2"));
  let mut test= String::from("test");
  test.push('c');
  println!("{}", test);
  fn bar<'a, 'b:'a>(x: &'a str, y: &'b str) -> &'a str {
    if false {
      x
    } else {
      y
    }
  }

  println!("{}",bar("test1", "test2"));


}