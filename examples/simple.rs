extern crate panic_cuddle;

use panic_cuddle::cuddle;
use std::panic;

pub fn main() {
    panic::set_hook(cuddle());

    panic!("Wanna hug");
}
