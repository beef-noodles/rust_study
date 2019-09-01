// mod basic;
// mod enums;
// mod cli;
// mod hash;
// mod matchs;
// mod traits;
// mod genric;
mod borrow;
mod a;
use a::b::c::d;

fn main() {
    borrow::run();
    // cli::run();
    d::print_ddd();
    println!("hello");
}
