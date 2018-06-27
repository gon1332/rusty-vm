use vm::defs::{Register, Opcode};

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Instruction {
    opcode: Opcode,
    register_a: Option<Register>,
    register_b: Option<Register>,
    operand: Option<i32>,
}

#[allow(unused)]
impl Instruction {
    pub fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn get_register_a(&self) -> Option<Register> {
        self.register_a
    }

    pub fn get_register_b(&self) -> Option<Register> {
        self.register_b
    }

    pub fn get_operand(&self) -> Option<i32> {
        self.operand
    }

    pub fn push(operand: i32) -> Instruction {
        Instruction { opcode: Opcode::PSH, register_a: None, register_b: None, operand: Some(operand) }
    }

    pub fn add() -> Instruction {
        Instruction { opcode: Opcode::ADD, register_a: None, register_b: None, operand: None }
    }

    pub fn pop() -> Instruction {
        Instruction { opcode: Opcode::POP, register_a: None, register_b: None, operand: None }
    }

    pub fn set(register: Register, operand: i32) -> Instruction {
        Instruction { opcode: Opcode::SET, register_a: Some(register), register_b: None, operand: Some(operand) }
    }

    pub fn mov(register_a: Register, register_b: Register) -> Instruction {
        Instruction { opcode: Opcode::MOV, register_a: Some(register_a), register_b: Some(register_b), operand: None }
    }

    pub fn halt() -> Instruction {
        Instruction { opcode: Opcode::HLT, register_a: None, register_b: None, operand: None }
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Instruction) -> bool {
        self.opcode == other.opcode
            && self.register_a == other.register_a
            && self.register_b == other.register_b
            && self.operand == self.operand
    }
}
