pub mod enums;
// #[test]
// #[should_panic]
// fn it_works() {
//     assert!(false);

// }

// #[test]
// #[should_panic(expected = "assertion failed")]
// fn it_not_equal() {
//     assert_eq!("hello", "world");
// }

// #[warn(dead_code)]
// pub fn add_two (a: i32) -> i32 {
//     a + 2
// }
// #[test]
// fn test_add_two() {
//     assert_eq!(3, add_two(1))
// }
// #[test]
// #[ignore]
// fn expensive_test() {
//     // code
// }

pub fn add_three(a: i32) -> i32 {
    a + 3
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works_for_test() {
//         assert_eq!(5, add_three(2));
//     }
// }
