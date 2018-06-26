pub mod defs;
use self::defs::{Register, Opcode};

pub mod instruction;
use self::instruction::Instruction;


pub struct RustVM {
    pub running: bool,
    stack: Vec<i32>,
    registers: Vec<i32>,
    program: Vec<Instruction>,
}

#[allow(unused)]
impl RustVM {
    pub fn new(program: Vec<Instruction>) -> RustVM {
        RustVM {
            running: true,
            stack:   Vec::new(),
            registers: vec![0; Register::TOTAL_REGISTERS as usize],
            program: program,
        }
    }

    pub fn print_status(&self) {
        println!("");
        println!("RustVM status");
        print!("stack: [");
        for i in &self.stack{
            print!("{} ", i);
        }
        println!("");
        println!("registers: ");
        println!("A:  {}", self.registers[Register::A as usize]);
        println!("B:  {}", self.registers[Register::B as usize]);
        println!("C:  {}", self.registers[Register::C as usize]);
        println!("Pc: {}", self.registers[Register::PC as usize]);
    }

    pub fn fetch(&self) -> Instruction {
        self.program[self.registers[Register::PC as usize] as usize]
    }

    pub fn decode(&mut self, instr: &Instruction) {
        match instr.get_opcode() {
            Opcode::HLT => {
                self.running = false;
            }
            Opcode::PSH => {
                let operand = match instr.get_operand() {
                    Some(operand) => operand,
                    None => panic!("invalid instruction: psh should have an operand")
                };
                self.stack.push(operand);
            }
            Opcode::POP => {
                match self.stack.pop() {
                    Some(value) => { println!("Popped {}", value) },
                    None => panic!("stack error: Pop on empty stack")
                };
            }
            Opcode::ADD => {
                let value_a = match self.stack.pop() {
                    Some(value) => value,
                    None => panic!("stack error: Pop on empty stack")
                };

                let value_b = match self.stack.pop() {
                    Some(value) => value,
                    None => panic!("stack error: Pop on empty stack")
                };

                let result = value_a + value_b;
                self.stack.push(result);
            }
            Opcode::SET => {
                let register = match instr.get_register() {
                    Some(register) => register,
                    None => panic!("invalid instruction: set should have a register")
                };
                let operand = match instr.get_operand() {
                    Some(operand) => operand,
                    None => panic!("invalid instruction: set should have an operand")
                };
                self.registers[register as usize] = operand;
            }
        }
    }

    pub fn step(&mut self) {
        self.registers[Register::PC as usize] += 1;
    }
}