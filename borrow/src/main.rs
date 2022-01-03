use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let source = read_to_string("./src/main.rs").expect("路径不对");
    let mut files = HashMap::new();
    files.insert("README", source.clone());
    files.insert("README2", source);

    let files_ref = &files;
    let files_ref2 = &files;

    print_borrowed_map(files_ref);
    println!("=============");
    print_borrowed_map(files_ref2);
}

fn print_borrowed_map(map: &HashMap<&str, String>) {
    println!("{:?}", map)
}
