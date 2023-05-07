Just notes as I build the derrrb temper
===

Writing Rust for an embedded system
---
A Hardware Abstraction Layer (HAL) is required to run Rust on AVR microcontrollers
and other common boards. To use this we need to use the nightly Rust compiler.

We can't use Rust's standard library or even a main f(x) as we are running on bare metal (no Operating System).
```rust
#![no_std]
#![no_main]
```

Instead of using the `main` f(x) as the entry point for our program, we will use a macro from the Hardware Abstraction Layer:
```rust
#[arduino_hal::entry]
```
This will specify the programs entry point.




Importing Rust created binary to the microcontroller
---
1. List open USB ports: `lsusb`
2. Set the serial com port for `ravedude`: `export RAVEDUDE_PORT=/dev/ttyUSB1`
3. Upload the program to the board's flash memory: `cargo run`
