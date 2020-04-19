
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


use std::ops::Deref;


struct MyBox<T>(T);

impl <T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl <T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("Hello World!");
}
