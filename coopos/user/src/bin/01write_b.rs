#![no_std]
#![no_main]

use coopos_user::sched_yield;

#[macro_use]
extern crate coopos_user;

const WIDTH: usize = 10;
const HEIGHT: usize = 2;

#[no_mangle]
fn main() -> i32 {
    for i in 0..HEIGHT {
        for _ in 0..WIDTH {
            print!("B");
        }
        println!(" [{}/{}]", i + 1, HEIGHT);
        sched_yield();
    }
    println!("Test write_b OK!");
    0
}
