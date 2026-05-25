#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("神秘人");
    
    let a = 10;
    let b = 20;
    println!("10 + 20 = {}", a + b);
    0
}