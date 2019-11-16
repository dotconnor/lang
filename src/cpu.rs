use std::convert::TryInto;

use crate::create_memory::create_memory;
use crate::instructions::Instructions;

const NUM_OF_REGISTERS: u8 = 12;

#[derive(Debug)]
pub struct CPU {
  memory: Vec<u8>,
  register_names: [&'static str; NUM_OF_REGISTERS as usize],
  registers: Vec<u8>,
  stack_frame_size: u16,
}

fn u16_to_u8(value: u16) -> [u8; 2] {
  value.to_be_bytes()
}

fn u8_to_u16(value: &[u8]) -> u16 {
  ((value[0] as u16) << 8) | value[1] as u16
}

impl CPU {
  pub fn new(memory: Vec<u8>) -> Self {
    let register_names = [
      "ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "sp", "fp",
    ];

    let memory_length = memory.len();

    let mut c = CPU {
      memory,
      register_names: register_names,
      registers: create_memory((NUM_OF_REGISTERS * 2).into()),
      stack_frame_size: 0,
    };

    c.set_register_name("sp", (memory_length - 1 - 1).try_into().unwrap());
    c.set_register_name("fp", (memory_length - 1 - 1).try_into().unwrap());

    return c;
  }

  #[cfg(debug_assertions)]
  #[allow(dead_code)]
  pub fn internal_debug(&self) {
    println!("{:?}", self);
  }

  #[cfg(debug_assertions)]
  #[allow(dead_code)]
  pub fn debug(&mut self) {
    for name in self.register_names.iter() {
      println!("{: >3}:  0x{:0>4X}", name, self.get_register_name(name));
    }
    println!();
  }

  #[cfg(debug_assertions)]
  #[allow(dead_code)]
  pub fn view_memory_at(&mut self, address: usize, n: Option<usize>) {
    let mut next_n_bytes = String::from(format!("0x{:0>4X}:", address));
    for i in 0..n.unwrap_or(8) {
      next_n_bytes.push_str(&format!(" 0x{:0>2X}", self.memory[address + i]));
    }
    println!("{}", next_n_bytes);
  }

  fn get_memory_location_of_register(&self, register: &'static str) -> u8 {
    (self
      .register_names
      .iter()
      .position(|&r| r == register)
      .unwrap()
      * 2) as u8
  }

  pub fn get_register(&self, location: u8) -> u16 {
    u8_to_u16(&self.registers[(location as usize)..(location + 2) as usize])
  }

  pub fn get_register_name(&self, name: &'static str) -> u16 {
    let index = self.get_memory_location_of_register(name);
    self.get_register(index)
  }

  pub fn set_register(&mut self, location: u8, value: u16) {
    let values = u16_to_u8(value);
    self.registers[location as usize] = values[0];
    self.registers[(location + 1) as usize] = values[1];
  }

  pub fn set_register_name(&mut self, name: &'static str, value: u16) {
    let index = self.get_memory_location_of_register(name);
    self.set_register(index, value);
  }

  fn get16(buf: &Vec<u8>, location: usize) -> u16 {
    u8_to_u16(&buf[location..location + 2])
  }

  pub fn fetch(&mut self) -> u8 {
    let next_instruction_address = self.get_register_name("ip");
    let instruction = self.memory[next_instruction_address as usize];
    self.set_register_name("ip", (next_instruction_address + 1) as u16);
    instruction
  }

  pub fn fetch16(&mut self) -> u16 {
    let next_instruction_address = self.get_register_name("ip") as usize;
    let instruction = CPU::get16(&self.memory, next_instruction_address);
    self.set_register_name("ip", (next_instruction_address + 2) as u16);
    instruction
  }

  pub fn fetch_register_index(&mut self) -> u8 {
    (self.fetch() % NUM_OF_REGISTERS) * 2
  }

  pub fn get_memory(&self, address: u16) -> u16 {
    CPU::get16(&self.memory, address as usize)
  }

  pub fn set_memory(&mut self, address: u16, value: u16) {
    let values = u16_to_u8(value);
    self.memory[address as usize] = values[0];
    self.memory[(address + 1) as usize] = values[1];
  }

  fn push(&mut self, value: u16) {
    let sp_address = self.get_register_name("sp");
    self.set_memory(sp_address, value);
    self.set_register_name("sp", sp_address - 2);
    self.stack_frame_size += 2;
  }

  fn pop(&mut self) -> u16 {
    let next_sp_address = self.get_register_name("sp") + 2;
    self.set_register_name("sp", next_sp_address);
    self.stack_frame_size -= 2;
    self.get_memory(next_sp_address)
  }

  fn push_state(&mut self) {
    self.push(self.get_register_name("r1"));
    self.push(self.get_register_name("r2"));
    self.push(self.get_register_name("r3"));
    self.push(self.get_register_name("r4"));
    self.push(self.get_register_name("r5"));
    self.push(self.get_register_name("r6"));
    self.push(self.get_register_name("r7"));
    self.push(self.get_register_name("r8"));
    self.push(self.get_register_name("ip"));
    self.push(self.stack_frame_size + 2);

    self.set_register_name("fp", self.get_register_name("sp"));
    self.stack_frame_size = 0;
  }

  fn pop_state(&mut self) {
    let frame_pointer_address = self.get_register_name("fp");
    self.set_register_name("sp", frame_pointer_address);

    self.stack_frame_size = self.pop();
    let stack_frame_size = self.stack_frame_size;

    let mut tmp = self.pop();
    self.set_register_name("ip", tmp);
    tmp = self.pop();
    self.set_register_name("r8", tmp);
    tmp = self.pop();
    self.set_register_name("r7", tmp);
    tmp = self.pop();
    self.set_register_name("r6", tmp);
    tmp = self.pop();
    self.set_register_name("r5", tmp);
    tmp = self.pop();
    self.set_register_name("r4", tmp);
    tmp = self.pop();
    self.set_register_name("r3", tmp);
    tmp = self.pop();
    self.set_register_name("r2", tmp);
    tmp = self.pop();
    self.set_register_name("r1", tmp);

    let n_args = self.pop();

    for _ in 0..n_args {
      self.pop();
    }

    self.set_register_name("fp", frame_pointer_address + stack_frame_size);
  }

  pub fn execute(&mut self, instruction: u8) {
    match instruction {
      // move literal into register
      Instructions::MOV_LIT_REG => {
        let literal = self.fetch16();
        let register = self.fetch_register_index();
        self.set_register(register, literal);
      }

      Instructions::MOV_REG_REG => {
        let register_from = self.fetch_register_index();
        let register_to = self.fetch_register_index();
        let value = self.get_register(register_from);
        self.set_register(register_to, value);
      }

      Instructions::MOV_REG_MEM => {
        let register_from = self.fetch_register_index();
        let address = self.fetch16();
        let value = self.get_register(register_from);
        self.set_memory(address, value);
      }

      Instructions::MOV_MEM_REG => {
        let address = self.fetch16();
        let register_to = self.fetch_register_index();
        let value = self.get_memory(address);
        self.set_register(register_to, value);
      }

      Instructions::ADD_REG_REG => {
        let r1 = self.fetch() as usize;
        let r2 = self.fetch() as usize;

        let register_value_1 = CPU::get16(&self.registers, r1 * 2);
        let register_value_2 = CPU::get16(&self.registers, r2 * 2);

        self.set_register_name("acc", register_value_1 as u16 + register_value_2 as u16);
      }

      Instructions::JMP_NOT_EQ => {
        let value = self.fetch16();
        let address = self.fetch16();

        if value != self.get_register_name("acc") {
          self.set_register_name("ip", address);
        }
      }

      Instructions::PSH_LIT => {
        let value = self.fetch16();
        self.push(value);
      }

      Instructions::PSH_REG => {
        let register_index = self.fetch_register_index();
        self.push(self.get_register(register_index));
      }

      Instructions::POP => {
        let register_index = self.fetch_register_index();
        let value = self.pop();
        self.set_register(register_index, value);
      }

      Instructions::CAL_LIT => {
        let address = self.fetch16();
        self.push_state();
        self.set_register_name("ip", address);
      }

      Instructions::CAL_REG => {
        let register_index = self.fetch_register_index();
        let address = self.get_register(register_index);
        self.push_state();
        self.set_register_name("ip", address);
      }

      Instructions::RET => {
        self.pop_state();
      }

      _ => {}
    }
  }

  pub fn step(&mut self) {
    let instruction = self.fetch();
    self.execute(instruction);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_cpu() {
    let c = CPU::new(create_memory(32));
    assert_eq!(c.memory.len(), 32);
  }

  #[test]
  fn cpu_registers() {
    let mut c = CPU::new(create_memory(32));
    c.set_register_name("r1", 16);
    let r = c.get_register_name("r1");
    assert_eq!(r, 16);
  }

  #[test]
  fn memory_calls() {
    let mut c = CPU::new(create_memory(256));
    c.set_memory(0x1, 24);
    assert_eq!(c.get_memory(0x1), 24);
  }
}
