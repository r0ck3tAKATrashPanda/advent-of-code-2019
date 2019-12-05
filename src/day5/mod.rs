//! Day 5 solution for Advent of Code. This is supposed to be an extension of Day 2,
//! but I am doing a major refactor here to make the CPU more user friendly instead of
//! the least amount of code

macro_rules! address_or_value {
    ($iter_name:ident, $cpu:ident, $expression:expr) => {
        if $iter_name.next() == Some('1') {
            $cpu.get_memory($expression)
        } else {
            $cpu.get_value($expression)
        }
    };
}

type CpuResult<T> = std::result::Result<T, CpuError>;

#[derive(Debug, Clone, Copy)]
enum CpuError {
    InvalidOpcode(isize, usize),
    InvalidLastInstruction,
    InvalidUserInput,
}

#[derive(Debug, Clone)]
struct CPU {
    memory: Vec<isize>,
    instruction_pointer: usize,
    last_instruction: Option<Instruction>,
}

impl CPU {
    pub fn new(program: &str) -> CPU {
        CPU {
            memory: program
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect(),
            instruction_pointer: 0,
            last_instruction: None,
        }
    }

    pub fn run(&mut self) -> CpuResult<()> {
        loop {
            match self.parse()? {
                Instruction::Add(left, right, location) => {
                    println!("Add : {} + {} @ {}", left, right, location);
                    self.last_instruction = Some(Instruction::Add(left, right, location));
                    self.set_memory(location as usize, left + right);
                }
                Instruction::Mult(left, right, location) => {
                    println!("Mult : {} * {} @ {}", left, right, location);
                    self.last_instruction = Some(Instruction::Mult(left, right, location));
                    self.set_memory(location as usize, left * right);
                }
                Instruction::In(location) => {
                    println!("In : @ {}", location);
                    self.last_instruction = Some(Instruction::In(location));
                    self.set_memory(location as usize, get_input()?);
                }
                Instruction::Out(value) => {
                    println!("Out : {}", value);
                    self.last_instruction = Some(Instruction::Out(value));
                    println!("{}", value as usize);
                }
                Instruction::JumpIfTrue(value, new_ip) => {
                    println!("JIT : {} to {}", value, new_ip);
                    self.last_instruction = Some(Instruction::JumpIfTrue(value, new_ip));
                    if value != 0 {
                        self.instruction_pointer = new_ip as usize;
                    }
                }
                Instruction::JumpIfFalse(value, new_ip) => {
                    println!("JIF : {} to {}", value, new_ip);
                    self.last_instruction = Some(Instruction::JumpIfFalse(value, new_ip));
                    if value == 0 {
                        self.instruction_pointer = new_ip as usize;
                    }
                }
                Instruction::LessThan(left, right, location) => {
                    println!("LT : {} < {} @ {}", left, right, location);
                    self.last_instruction = Some(Instruction::LessThan(left, right, location));
                    if left < right {
                        self.set_memory(location as usize, 1);
                    } else {
                        self.set_memory(location as usize, 0);
                    }
                }
                Instruction::Equal(left, right, location) => {
                    println!("EQ : {} < {} @ {}", left, right, location);
                    self.last_instruction = Some(Instruction::Equal(left, right, location));
                    if left == right {
                        self.set_memory(location as usize, 1);
                    } else {
                        self.set_memory(location as usize, 0);
                    }
                }
                Instruction::Halt => {
                    self.last_instruction = Some(Instruction::Halt);
                    return Ok(());
                }
            }
            self.increment_ip()?;
        }
    }

    fn parse(&self) -> CpuResult<Instruction> {
        Instruction::parse(&self)
    }

    fn increment_ip(&mut self) -> CpuResult<()> {
        if let Some(last_instr) = self.last_instruction {
            match last_instr {
                Instruction::Add(_, _, _) => {
                    self.instruction_pointer += 4;
                }
                Instruction::Mult(_, _, _) => {
                    self.instruction_pointer += 4;
                }
                Instruction::In(_) => {
                    self.instruction_pointer += 2;
                }
                Instruction::Out(_) => {
                    self.instruction_pointer += 2;
                }
                Instruction::JumpIfTrue(value, _) => {
                    if value != 0 {
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Instruction::JumpIfFalse(value, _) => {
                    if value == 0 {
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Instruction::LessThan(_, _, _) => {
                    self.instruction_pointer += 4;
                }
                Instruction::Equal(_, _, _) => {
                    self.instruction_pointer += 4;
                }
                Instruction::Halt => {
                    self.instruction_pointer += 1;
                }
            }
            Ok(())
        } else {
            Err(CpuError::InvalidLastInstruction)
        }
    }

    /// Get the value at a memory address
    fn get_memory(&self, address: usize) -> isize {
        self.memory[address]
    }

    /// Convienient wrapper around `get_memory` that will get a value instead of a position if needed
    fn get_value(&self, address: usize) -> isize {
        self.get_memory(self.get_memory(address) as usize)
    }

    fn set_memory(&mut self, address: usize, value: isize) {
        self.memory[address] = value;
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Add(isize, isize, isize),
    Mult(isize, isize, isize),
    In(isize),
    Out(isize),
    JumpIfTrue(isize, isize),
    JumpIfFalse(isize, isize),
    LessThan(isize, isize, isize),
    Equal(isize, isize, isize),
    Halt,
}

impl Instruction {
    fn parse(cpu: &CPU) -> CpuResult<Instruction> {
        let mem = cpu.get_memory(cpu.instruction_pointer);
        let flags = (mem / 100).to_string();
        let mut flags_iter = flags.chars().rev();
        let instr = match mem % 100 {
            1 => Instruction::Add(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                cpu.get_memory(cpu.instruction_pointer + 3),
            ),
            2 => Instruction::Mult(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                cpu.get_memory(cpu.instruction_pointer + 3),
            ),
            3 => Instruction::In(cpu.get_memory(cpu.instruction_pointer + 1)),
            4 => Instruction::Out(address_or_value!(
                flags_iter,
                cpu,
                cpu.instruction_pointer + 1
            )),
            5 => Instruction::JumpIfTrue(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
            ),
            6 => Instruction::JumpIfFalse(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
            ),
            7 => Instruction::LessThan(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                cpu.get_memory(cpu.instruction_pointer + 3),
            ),
            8 => Instruction::Equal(
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 1),
                address_or_value!(flags_iter, cpu, cpu.instruction_pointer + 2),
                cpu.get_memory(cpu.instruction_pointer + 3),
            ),
            99 => Instruction::Halt,
            _ => {
                return Err(CpuError::InvalidOpcode(
                    cpu.get_memory(cpu.instruction_pointer),
                    cpu.instruction_pointer,
                ))
            }
        };

        Ok(instr)
    }
}

fn get_input() -> CpuResult<isize> {
    use std::io;

    let mut input = String::new();

    if let Err(_) = io::stdin().read_line(&mut input) {
        return Err(CpuError::InvalidUserInput);
    }
    let parse_input = input.trim().parse::<isize>();
    match parse_input {
        Err(_) => return Err(CpuError::InvalidUserInput),
        Ok(val) => return Ok(val),
    }
}

#[aoc(day5, part1)]
fn d5p1(input: &str) -> isize {
    let mut cpu = CPU::new(input);
    if let Err(error) = cpu.run() {
        println!("ERROR : {:?}", error);
        0
    } else {
        cpu.get_memory(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut cpu = CPU::new("1002,4,3,4,33,99");
        println!("{:?}", cpu.run());
        assert_eq!(cpu.get_memory(4), 99);
    }

    #[test]
    fn test2() {
        let mut cpu = CPU::new("1101,100,-1,4,0");
        println!("{:?}", cpu.run());
        assert_eq!(cpu.get_memory(4), 99);
    }
}
