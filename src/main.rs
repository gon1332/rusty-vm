mod vm;
use vm::RustVM;
use vm::defs::Register;
use vm::instruction::Instruction;

fn main() {
    let program = vec![
        Instruction::push(5),
        Instruction::push(6),
        Instruction::add(),
        Instruction::pop(),
        Instruction::set(Register::A, 23),
        Instruction::halt(),
    ];

    let mut vm = RustVM::new(program);

    while vm.running == true {
        // vm.print_status();
        let instr = vm.fetch();
        vm.decode(&instr);
        vm.step();
    }
}
