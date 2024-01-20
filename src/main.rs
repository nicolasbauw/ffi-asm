use core::arch::global_asm;

global_asm!(include_str!("addition.s"));
extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 10;
    let b = 20;
    let r= unsafe { add_numbers(a, b) };
    println!("Result: {r}");
}