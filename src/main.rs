use core::arch::global_asm;

#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("aarch64/addition.s"));

#[cfg(target_arch = "arm")]
global_asm!(include_str!("armv6/addition.s"));

#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("riscv/addition.s"));

extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 10;
    let b = 20;
    let r= unsafe { add_numbers(a, b) };
    println!("Result: {r}");
}
