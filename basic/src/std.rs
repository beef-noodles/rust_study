pub fn run() -> i32 {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("Failed to read line");
  let num = input.trim().parse::<i32>().unwrap();
  println!("您输入的数字是：{}", num);
  num
}
