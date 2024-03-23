#![no_std]
#![no_main]

use addressos_user::*;

#[no_mangle]
fn main() -> i32 {
    let current_timer = get_time();
    let wait_for = current_timer + 3000;
    while get_time() < wait_for {
        sched_yield();
    }
    println!("Test sleep OK!");
    0
}
