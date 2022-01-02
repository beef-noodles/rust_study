extern crate rust_study;

#[test]
fn it_works_in_tests() {
    assert_eq!(5, rust_study::add_three(2));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_in_mod() {
        assert_eq!(1000, rust_study::add_three(997));
    }
}
