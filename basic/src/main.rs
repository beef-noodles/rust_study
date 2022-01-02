// mod basic;
// mod enums;
// mod cli;
// mod hash;
// mod matchs;
// mod traits;
// mod genric;
// mod borrow;
// mod a;
// use a::b::c::d;
// mod file;
// mod std;
mod impls;
fn main() {
    // borrow::run();
    // cli::run();
    // d::print_ddd();
    // let file_name = "foobar.rs";
    // match d::find(file_name, '.') {
    //     None => println!("No file extension found."),
    //     Some(i) => println!("File extension is {}",&file_name[i+1..]),
    // }
    // file::file::read_file();
    // std::run();
    impls::run();
    println!("hello");
}
