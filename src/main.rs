mod cpu;
mod create_memory;
mod instructions;

use cpu::CPU;
use create_memory::create_memory;
use instructions::{Instructions, Registers};

fn main() {
  let mut i = 0;

  let mut get_next = |n: Option<usize>| {
    if let Some(p) = n {
      i = p;
      i
    } else {
      let r = i;
      i += 1;
      r
    }
  };

  let mut m = create_memory(128 * 128);

  m[get_next(None)] = Instructions::PSH_LIT;
  m[get_next(None)] = 0x33;
  m[get_next(None)] = 0x33;

  m[get_next(None)] = Instructions::PSH_LIT;
  m[get_next(None)] = 0x22;
  m[get_next(None)] = 0x22;

  m[get_next(None)] = Instructions::PSH_LIT;
  m[get_next(None)] = 0x11;
  m[get_next(None)] = 0x11;

  m[get_next(None)] = Instructions::MOV_LIT_REG;
  m[get_next(None)] = 0x12;
  m[get_next(None)] = 0x34;
  m[get_next(None)] = Registers::R1;

  m[get_next(None)] = Instructions::MOV_LIT_REG;
  m[get_next(None)] = 0x56;
  m[get_next(None)] = 0x78;
  m[get_next(None)] = Registers::R4;

  m[get_next(None)] = Instructions::PSH_LIT;
  m[get_next(None)] = 0x00;
  m[get_next(None)] = 0x00;

  m[get_next(None)] = Instructions::CAL_LIT;
  m[get_next(None)] = 0x30;
  m[get_next(None)] = 0x00;

  m[get_next(None)] = Instructions::PSH_LIT;
  m[get_next(None)] = 0x44;
  m[get_next(None)] = 0x44;

  get_next(Some(0x3000));

  m[get_next(None)] = Instructions::PSH_LIT;
  m[get_next(None)] = 0x01;
  m[get_next(None)] = 0x02;

  m[get_next(None)] = Instructions::PSH_LIT;
  m[get_next(None)] = 0x03;
  m[get_next(None)] = 0x04;

  m[get_next(None)] = Instructions::PSH_LIT;
  m[get_next(None)] = 0x05;
  m[get_next(None)] = 0x06;

  m[get_next(None)] = Instructions::MOV_LIT_REG;
  m[get_next(None)] = 0x07;
  m[get_next(None)] = 0x08;
  m[get_next(None)] = Registers::R1;

  m[get_next(None)] = Instructions::MOV_LIT_REG;
  m[get_next(None)] = 0x09;
  m[get_next(None)] = 0x0A;
  m[get_next(None)] = Registers::R8;

  m[get_next(None)] = Instructions::RET;

  let mut c = CPU::new(m);

  for _ in 0..14 {
    c.step();
  }
  c.debug();
}
