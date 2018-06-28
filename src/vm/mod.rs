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
                let register = match instr.get_register_a() {
                    Some(register) => register,
                    None => panic!("invalid instruction: pop <reg>")
                };
                match self.stack.pop() {
                    Some(value) => { self.registers[register as usize] = value; },
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
                let register = match instr.get_register_a() {
                    Some(register) => register,
                    None => panic!("invalid instruction: set should have a register")
                };
                let operand = match instr.get_operand() {
                    Some(operand) => operand,
                    None => panic!("invalid instruction: set should have an operand")
                };
                self.registers[register as usize] = operand;
            }
            Opcode::MOV => {
                let register_a = match instr.get_register_a() {
                    Some(register) => register,
                    None => panic!("invalid instruction: mov should have a destination register")
                };
                let register_b = match instr.get_register_b() {
                    Some(register) => register,
                    None => panic!("invalid instruction: mov should have a source register")
                };
                self.registers[register_a as usize] = self.registers[register_b as usize];
            }
            Opcode::LOG => {
                match instr.get_register_a() {
                    Some(register) => println!("{}", self.registers[register as usize]),
                    None => panic!("invalid instruction: log <reg>")
                }
            }
        }
    }

    pub fn step(&mut self) {
        self.registers[Register::PC as usize] += 1;
    }
}
