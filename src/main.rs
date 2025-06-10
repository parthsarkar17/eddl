pub mod ir;

use ir::{constructs::port::Port, primitives::structures::Gate};

fn main() {
    let port = Port::new(0, 1, "port", true);

    println!("Hello, world!");
}
