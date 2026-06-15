#![no_std]
#![no_main]

use user_lib::println;

#[unsafe(no_mangle)]
fn main() {
    for i in 0..10 {
        println!("Number: {}", i);
    }
}