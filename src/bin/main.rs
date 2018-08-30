extern crate riscv_isa;
use riscv_isa::decode::decode;

fn main() {
    println!("Hello, RISC-V!");

    let inst_reg = 1;
    let x = decode(inst_reg);
    println!("{:?}", x);
}
