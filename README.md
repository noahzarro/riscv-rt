
# `riscv-rt`

> Runtime / startup for RISC-V CPU's.

This project was forked from [this repository][repo]. This fork adds support for the Core Local Interrupt Controller (CLIC).

## Usage

### Entry Point
To define a function as an entry point, use the `#entry` macro. It checks if the function has the following signature: `[unsafe] fn([arg0: usize, ...]) -> !`. Then the function is exported under the name `main`. If another signature is desired, the `#entry` macro can be omitted, and the desired main function has to be exported under the `main` symbol.

However, if the `#entry` symbol is not used one has to take care that the runtime crate is actually included. This can be achieved by including a dummy interrupt handler.

### Interrupt Handlers
To specify an interrupt handler, the `#[interrupt_handler(arg)]` macro can be added to a rust function. It takes care of saving and restoring the current context.
The argument can be provided in 3 different forms:
#### Integer
If an integer `i` is provided, the handler function is exported as `int_i`, that directly corresponds to the entry of the vector table.

#### Enum Value
The PAC (peripheral access crate) crate provides an enum for all interrupts that are accessible in the system. If the name of an interrupt enum is provided (e.g.`#[interrupt_handler(UART0)]`), the handler function is exported with that name. The PAC provides for each interrupt a `PROVIDE(int_23 = UART0)` statement in its link file. Like this, the interrupt handler is mapped to its corresponding vector table entry.

#### No Argument
If no argument is provided, the interrupt handler is exported with the same name as the function. In this setup the the user has to provide a linker script entry `PROVIDE(int_i = my_handler_function)` where `i` is the interrupt number. It must be added in the `memory.x` linker script.

### Linker Script
The user needs to provide a linker script in the top level directory of its program folder, called `memory.x` it has to provide the following memory regions:

- REGION_TEXT
- REGION_RODAT
- REGION_DATA
- REGION_BSS
- REGION_HEAP
- REGION_STACK

If desired the start of the execution can be defined in a provided `_stext` entry.

### Interrupt Setup
Interrupts sill have to be enabled with the commands provided in the MAC (micro architecture crate).

## Background

This crate adds a startup code that sets up the interrupt vector with jump instructions to `int_0` to `int_264`. This could be extended or shrinked freely.
The crate also handles the correct setting up of the `mtvec` and `mtvt` registers.

## Features

The crate provides the following features:

### clic
The clic feature enables the usage of the core local interrupt controller (CLIC). It features a vector table. Since this is the main feature of this fork, it should not be used without it. If the clic is not used, the original implementation will work in a more stable way, since it has been tested on much more devices.

### nxti
The nxti feature makes use of the `mnxti` register. With this feature enabled, tail-chaining is used when handling interrupts. This optimizes out unnecessary interrupt context switches. If the nxti is used, all used interrupts need to be configured to non vectorized mode (set the shv bit to 0). This can be configured with commands provided by the MAC crate.

### s-mode
The s-mode is currently not supported in this fork, since there was no way to test it. Use the original repo for the s-mode feature.

## Limitations
This fork is not optimized and tested for 64bit systems. It might work, but no guarantees can be made. Furthermore, the libraries in the `bin` folder are currently only compiled for 32bit setups.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-risc-v-team
[repo]: https://github.com/rust-embedded/riscv-rt