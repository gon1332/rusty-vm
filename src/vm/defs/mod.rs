#[derive(Copy, Clone)]
pub enum Opcode {
    PSH = 0,
    ADD,
    POP,
    SET,
    HLT,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Register {
    A = 0,
    B,
    C,
    PC,
    TOTAL_REGISTERS,
}
