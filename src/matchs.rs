pub fn run() {
  let x = 1;
  match x {
    1 | 2 ..= 10 => println!("1-10"),
    _ => println!("其他"),
  }
  let c = 'w';
  match c {
    'a' ..= 'z' => println!("小写字母"),
    'A' ..= 'Z' => println!("大写字母"),
    _ => println!("其他字母")
  }
  let mut y = 5;
  match y {
    ref mut my => println!("mut ref: {}", my),
  }

  #[derive(Debug)]
  struct Person {
    name: Option<String>,
  }
  let name = "Steve".to_string();
  let z : Option<Person> = Some(Person{name: Some(name)});
  match z {
    Some(Person{ name: ref a @ Some(_), .. }) => println!("{:?}", a),
    _ => {}
  }
  let a = 4;
  let y = false;
  match a {
    4 | 5 if y => println!("yes"),
    _ => println!("no"),
  }

}