use std::str::FromStr;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Opcode {
    PSH = 0,
    ADD,
    POP,
    SET,
    MOV,
    HLT,
    LOG,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Register {
    A = 0,
    B,
    C,
    PC,
    TOTAL_REGISTERS,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Register, ()> {
        match s {
            "A" => Ok(Register::A),
            "B" => Ok(Register::B),
            "C" => Ok(Register::C),
            "PC" => Ok(Register::PC),
            _ => Err(()),
        }
    }
}
