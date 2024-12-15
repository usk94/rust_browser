#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

use noli::prelude::*;

fn main() {
    Api::write_string("hello world");
    println!("hello from println!");
    Api::exit(42);
}

entry_point!(main);
