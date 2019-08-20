use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  println!("args: {:?}", args);
  let command = args[1].clone();
  let name = "Guzhongren";
  if command == "hello" {
    println!("hello, {}! Nice to meet you ~", name);
  } else if command == "test" {
    println!("doing test");
    println!("test finished~")
  } else {
    println!(
      "Error: Error command :{}, please input corecte command",
      command
    );
  }
}
