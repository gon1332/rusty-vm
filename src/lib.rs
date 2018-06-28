use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

mod vm;
use vm::RustVM;
use vm::defs::Register;
use vm::instruction::Instruction;


pub struct Config {
    program_filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Usage: rustyvm <program_filename>");
        }

        let program_filename = args[1].clone();

        Ok(Config { program_filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.program_filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let mut vm = RustVM::new(parse(&contents));

    while vm.running == true {
        // vm.print_status();
        let instr = vm.fetch();
        vm.decode(&instr);
        vm.step();
    }

    Ok(())
}

pub fn parse(contents: &str) -> Vec<Instruction> {
    let mut program = Vec::new();

    for line in contents.lines() {
        // let line = line.trim();
        if line.contains("push") {
            let v: Vec<&str> = line.split(' ').collect();
            let operand = v[1].parse::<i32>().unwrap();
            program.push(Instruction::push(operand));
        }
        else if line.contains("add") {
            program.push(Instruction::add());
        }
        else if line.contains("pop") {
            program.push(Instruction::pop());
        }
        else if line.contains("set") {
            let v: Vec<&str> = line.split(' ').collect();
            let register = v[1].parse::<Register>().unwrap();
            let operand  = v[2].parse::<i32>().unwrap();
            program.push(Instruction::set(register, operand));
        }
        else if line.contains("mov") {
            let v: Vec<&str> = line.split(' ').collect();
            let register_a = v[1].parse::<Register>().unwrap();
            let register_b = v[2].parse::<Register>().unwrap();
            program.push(Instruction::mov(register_a, register_b));
        }
        else if line.contains("halt") {
            program.push(Instruction::halt());
        }
        else if line.contains("log") {
            let v: Vec<&str> = line.split(' ').collect();
            let register = v[1].parse::<Register>().unwrap();
            program.push(Instruction::log(register));
        }
        else {
            panic!("Illegal Instruction");
        }
    }

    program
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_instruction() {
        let contents = "push 3";
        let golden = vec![Instruction::push(3)];

        assert_eq!(
            golden,
            parse(contents)
        );
    }

    #[test]
    fn add_instruction() {
        let contents = "add";
        let golden = vec![Instruction::add()];

        assert_eq!(
            golden,
            parse(contents)
        );
    }

    #[test]
    fn pop_instruction() {
        let contents = "pop";
        let golden = vec![Instruction::pop()];

        assert_eq!(
            golden,
            parse(contents)
        );
    }

    #[test]
    fn set_instruction() {
        let contents = "set A 3";
        let golden = vec![Instruction::set(Register::A, 23)];

        assert_eq!(
            golden,
            parse(contents)
        );
    }

    #[test]
    fn mov_instruction() {
        let contents = "mov A C";
        let golden = vec![Instruction::mov(Register::A, Register::C)];

        assert_eq!(
            golden,
            parse(contents)
        );
    }

    #[test]
    fn halt_instruction() {
        let contents = "halt";
        let golden = vec![Instruction::halt()];

        assert_eq!(
            golden,
            parse(contents)
        );
    }

    #[test]
    fn log_instruction() {
        let contents = "log A";
        let golden = vec![Instruction::log(Register::A)];

        assert_eq!(
            golden,
            parse(contents)
        );
    }
}
