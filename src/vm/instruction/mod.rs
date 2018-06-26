use vm::defs::{Register, Opcode};

#[derive(Copy, Clone)]
pub struct Instruction {
    opcode: Opcode,
    register: Option<Register>,
    operand: Option<i32>,
}

#[allow(unused)]
impl Instruction {
    pub fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn get_register(&self) -> Option<Register> {
        self.register
    }

    pub fn get_operand(&self) -> Option<i32> {
        self.operand
    }

    pub fn push(operand: i32) -> Instruction {
        Instruction { opcode: Opcode::PSH, register: None, operand: Some(operand) }
    }

    pub fn add() -> Instruction {
        Instruction { opcode: Opcode::ADD, register: None, operand: None }
    }

    pub fn pop() -> Instruction {
        Instruction { opcode: Opcode::POP, register: None, operand: None }
    }

    pub fn set(register: Register, operand: i32) -> Instruction {
        Instruction { opcode: Opcode::SET, register: Some(register), operand: Some(operand) }
    }

    pub fn halt() -> Instruction {
        Instruction { opcode: Opcode::HLT, register: None, operand: None }
    }
}
