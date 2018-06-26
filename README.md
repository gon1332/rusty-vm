# rusty-vm
Simple virtual machine in Rust.

Inspired from [the Felix Angell's mac blog post](https://felixangell.com/blog/implementing-a-virtual-machine-in-c/)
rusty-vm is an educational project for me to learn Rust.

## Architecture

#### Registers
~~~asm
    A, B, C ; General purpose registers
    PC      ; Program Counter
~~~

#### ISA
~~~asm
    push <i32>      ; push <i32> value to stack
    add             ; pop the two top values from stack and push the result
    pop             ; pop the top value from the stack and print the value
    set <reg> <i32> ; set register <reg> with the <i32> value
    halt            ; stop the execution of the virtual machine
~~~


## Usage

After you clone the project:
~~~sh
$ git clone https://github.com/gon1332/rusty-vm.git
$ cd rusty-vm
~~~

Run it with:
~~~sh
$ cargo run --release examples/simple.asm
~~~

You can also try the test cases:
~~~sh
$ cargo test
~~~
