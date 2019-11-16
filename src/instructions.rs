pub struct Instructions;

impl Instructions {
  pub const MOV_LIT_REG: u8 = 0x10;
  pub const MOV_REG_REG: u8 = 0x11;
  pub const MOV_REG_MEM: u8 = 0x12;
  pub const MOV_MEM_REG: u8 = 0x13;
  pub const ADD_REG_REG: u8 = 0x14;
  pub const JMP_NOT_EQ: u8 = 0x15;
  pub const PSH_LIT: u8 = 0x17;
  pub const PSH_REG: u8 = 0x18;
  pub const POP: u8 = 0x1A;
  pub const CAL_LIT: u8 = 0x5E;
  pub const CAL_REG: u8 = 0x5F;
  pub const RET: u8 = 0x60;
}

pub struct Registers;

#[allow(dead_code)]
impl Registers {
  pub const IP: u8 = 0;
  pub const ACC: u8 = 1;
  pub const R1: u8 = 2;
  pub const R2: u8 = 3;
  pub const R3: u8 = 4;
  pub const R4: u8 = 5;
  pub const R5: u8 = 6;
  pub const R6: u8 = 7;
  pub const R7: u8 = 8;
  pub const R8: u8 = 9;
  pub const SP: u8 = 10;
  pub const FP: u8 = 11;
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::cpu::CPU;
  use crate::create_memory::create_memory;

  #[test]
  fn mov_lit_reg() {
    let mut m = create_memory(32);
    m[0] = Instructions::MOV_LIT_REG;
    m[1] = 0x12;
    m[2] = 0x34;
    m[3] = Registers::R1;
    let mut c = CPU::new(m);
    c.step();
    assert_eq!(c.get_register(Registers::R1 * 2), 0x1234);
  }

  #[test]
  fn mov_reg_reg() {
    let mut m = create_memory(32);

    m[0] = Instructions::MOV_LIT_REG;
    m[1] = 0x12;
    m[2] = 0x34;
    m[3] = Registers::R1;

    m[4] = Instructions::MOV_REG_REG;
    m[5] = Registers::R1;
    m[6] = Registers::R2;

    let mut c = CPU::new(m);
    c.step();
    assert_eq!(c.get_register(Registers::R1 * 2), 0x1234);
    c.step();
    assert_eq!(c.get_register(Registers::R2 * 2), 0x1234);
  }

  #[test]
  fn mov_reg_mem() {
    let mut m = create_memory(512);

    m[0] = Instructions::MOV_LIT_REG;
    m[1] = 0x12;
    m[2] = 0x34;
    m[3] = Registers::R1;

    m[4] = Instructions::MOV_REG_MEM;
    m[5] = Registers::R1;
    m[6] = 0x01;
    m[7] = 0x00;

    let mut c = CPU::new(m);
    c.step();
    assert_eq!(c.get_register(Registers::R1 * 2), 0x1234);
    c.step();
    assert_eq!(c.get_memory(0x0100), 0x1234);
  }

  #[test]
  fn mov_mem_reg() {
    let mut m = create_memory(512);

    m[0] = Instructions::MOV_LIT_REG;
    m[1] = 0x12;
    m[2] = 0x34;
    m[3] = Registers::R1;

    m[4] = Instructions::MOV_REG_MEM;
    m[5] = Registers::R1;
    m[6] = 0x01;
    m[7] = 0x00;

    m[8] = Instructions::MOV_MEM_REG;
    m[9] = 0x01;
    m[10] = 0x00;
    m[11] = Registers::R2;

    let mut c = CPU::new(m);
    c.step();
    assert_eq!(c.get_register(Registers::R1 * 2), 0x1234);
    c.step();
    assert_eq!(c.get_memory(0x0100), 0x1234);
    c.step();
    assert_eq!(c.get_register(Registers::R2 * 2), 0x1234);
  }

  #[test]
  fn add_reg_reg() {
    let mut i = 0;

    let mut get_next = || {
      let r = i;
      i += 1;
      r
    };
    let mut m = create_memory(512);
    m[get_next()] = Instructions::MOV_LIT_REG;
    m[get_next()] = 0x12;
    m[get_next()] = 0x34;
    m[get_next()] = Registers::R1;

    m[get_next()] = Instructions::MOV_LIT_REG;
    m[get_next()] = 0xAB;
    m[get_next()] = 0xCD;
    m[get_next()] = Registers::R2;

    m[get_next()] = Instructions::ADD_REG_REG;
    m[get_next()] = Registers::R1;
    m[get_next()] = Registers::R2;

    let mut c = CPU::new(m);
    c.step();
    assert_eq!(c.get_register(Registers::R1 * 2), 0x1234);
    c.step();
    assert_eq!(c.get_register(Registers::R2 * 2), 0xABCD);
    c.step();
    assert_eq!(c.get_register(Registers::ACC * 2), 0xBE01);
  }

  #[test]
  fn jmp_not_eq() {
    let mut i = 0;

    let mut get_next = || {
      let r = i;
      i += 1;
      r
    };
    let mut m = create_memory(512);

    m[get_next()] = Instructions::MOV_MEM_REG;
    m[get_next()] = 0x01;
    m[get_next()] = 0x00;
    m[get_next()] = Registers::R1;

    m[get_next()] = Instructions::MOV_LIT_REG;
    m[get_next()] = 0x00;
    m[get_next()] = 0x01;
    m[get_next()] = Registers::R2;

    m[get_next()] = Instructions::ADD_REG_REG;
    m[get_next()] = Registers::R1;
    m[get_next()] = Registers::R2;

    m[get_next()] = Instructions::MOV_REG_MEM;
    m[get_next()] = Registers::ACC;
    m[get_next()] = 0x01;
    m[get_next()] = 0x00;

    m[get_next()] = Instructions::JMP_NOT_EQ;
    m[get_next()] = 0x00;
    m[get_next()] = 0x03;
    m[get_next()] = 0x00;
    m[get_next()] = 0x00;

    let mut c = CPU::new(m);

    for i in 0..3 {
      c.step();
      assert_eq!(c.get_memory(0x0100), i);
      c.step();
      assert_eq!(c.get_register_name("r2"), 1);
      c.step();
      assert_eq!(c.get_register_name("acc"), i + 1);
      c.step();
      assert_eq!(c.get_memory(0x0100), i + 1);
      c.step();
      let ip = if i == 2 { 20 } else { 0 };
      assert_eq!(c.get_register_name("ip"), ip);
    }
  }

  #[test]
  fn push_pop() {
    let mut i = 0;

    let mut get_next = || {
      let r = i;
      i += 1;
      r
    };
    let mut m = create_memory(512);

    m[get_next()] = Instructions::MOV_LIT_REG;
    m[get_next()] = 0x51;
    m[get_next()] = 0x51;
    m[get_next()] = Registers::R1;

    m[get_next()] = Instructions::MOV_LIT_REG;
    m[get_next()] = 0x42;
    m[get_next()] = 0x42;
    m[get_next()] = Registers::R2;

    m[get_next()] = Instructions::PSH_REG;
    m[get_next()] = Registers::R1;

    m[get_next()] = Instructions::PSH_REG;
    m[get_next()] = Registers::R2;

    m[get_next()] = Instructions::POP;
    m[get_next()] = Registers::R1;

    m[get_next()] = Instructions::POP;
    m[get_next()] = Registers::R2;

    let mut c = CPU::new(m);

    for _ in 0..6 {
      c.step();
    }

    assert_eq!(c.get_register_name("r1"), 0x4242);
    assert_eq!(c.get_register_name("r2"), 0x5151);
  }

  #[test]
  fn call_stack() {
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

    c.step();
    assert_eq!(c.get_memory(128 * 128 - 2), 0x3333); // push 1
    c.step();
    assert_eq!(c.get_memory(128 * 128 - 4), 0x2222); // push 2
    c.step();
    assert_eq!(c.get_memory(128 * 128 - 6), 0x1111); // push 3
    c.step();
    assert_eq!(c.get_register_name("r1"), 0x1234);
    c.step();
    assert_eq!(c.get_register_name("r4"), 0x5678);
    c.step();
    assert_eq!(c.get_memory(128 * 128 - 8), 0x0000); // # of params
    c.step();
    assert_eq!(c.get_register_name("ip"), 0x3000);
    assert_eq!(c.get_memory(128 * 128 - 10), 0x1234); // r1 saved to stack
    assert_eq!(c.get_memory(128 * 128 - 16), 0x5678); // r4 saved to stack
    assert_eq!(c.get_memory(128 * 128 - 26), 23); // return address
    assert_eq!(c.get_memory(128 * 128 - 28), 28); // stack size
    c.step();
    assert_eq!(c.get_memory(128 * 128 - 30), 0x0102); // sub call push 1
    c.step();
    assert_eq!(c.get_memory(128 * 128 - 32), 0x0304); // sub call push 2
    c.step();
    assert_eq!(c.get_memory(128 * 128 - 34), 0x0506); // sub call push 3
    c.step();
    assert_eq!(c.get_register_name("r1"), 0x0708); // sub call move reg
    c.step();
    assert_eq!(c.get_register_name("r8"), 0x090A); // sub call move reg
    c.step();
    assert_eq!(c.get_register_name("r1"), 0x1234);
    assert_eq!(c.get_register_name("r4"), 0x5678);
    c.step();
    assert_eq!(c.get_memory(128 * 128 - 8), 0x4444); // push 4
  }
}
