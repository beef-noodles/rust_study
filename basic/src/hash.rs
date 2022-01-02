use std::collections::HashMap;


pub fn run() {
  let mut come_from = HashMap::new();
  come_from.insert("WaySLOG", "HeBei");
  come_from.insert("Marisa", "U.S.");
  come_from.insert("Mike", "HuoGuo");
  if !come_from.contains_key("elton") {
    println!("Oh, 我们查了{}个人，但是可怜的Elton猫还是无家可归", come_from.len());
  }

  come_from.remove("Mike");

  let who = ["MoGu", "Marisa"];
  for person in &who {
    match come_from.get(person) {
      Some(location) => println!("{} 来自：{}", person, location),
      None => println!("{}也无家可归啊。", person),
    }
  }

  for person in &come_from {
    println!("{:?} 来自{}", person, person.1);
  }

  println!("entry");
  let mut letters = HashMap::new();
  for ch in "a short treatise on fungi".chars() {
    let counter = letters.entry(ch).or_insert(0);
    println!("{}", counter);
    *counter += 1;
  }
  assert_eq!(letters[&'s'], 2);


}