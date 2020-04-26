
//! My Crate name
//!
//! `my_crate_name` is test for studying
/// Add one for the number given
/// 
/// #Example
///
/// ```rust
///let five = 5
///assert_eq!(6, rust_study::add_one(five));
/// ```
///
// ub fn add_one(x: i32) -> i32 {
//    x + 1


// use std::ops::Deref;
// 
// 
// struct MyBox<T>(T);
// 
// impl <T> MyBox<T>{
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// impl <T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }
// 
// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);
//     assert_eq!(5, x);
//     assert_eq!(5, *y);
//     println!("Hello World!");
// }

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (sender, reviver) = mpsc::channel();
    let sender1 = sender.clone();
    thread::spawn(move|| {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            sender1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move|| {
        let vals = vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
            String::from("D"),
        ];
        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for r in reviver {
        println!("Got {}", r);
    }
}
