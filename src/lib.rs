//! Minimal startup / runtime for RISC-V CPU's
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.59 and up. It *might*
//! compile with older versions but that may change in any new patch release.
//!
//! # Features
//!
//! This crate provides
//!
//! - Before main initialization of the `.bss` and `.data` sections.
//!
//! - `#[entry]` to declare the entry point of the program
//! - `#[pre_init]` to run code *before* `static` variables are initialized
//!
//! - A linker script that encodes the memory layout of a generic RISC-V
//!   microcontroller. This linker script is missing some information that must
//!   be supplied through a `memory.x` file (see example below). This file
//!   must be supplied using rustflags and listed *before* `link.x`. Arbitrary
//!   filename can be use instead of `memory.x`.
//!
//! - A `_sheap` symbol at whose address you can locate a heap.
//!
//! - Support for a runtime in supervisor mode, that can be bootstrapped via [Supervisor Binary Interface (SBI)](https://github.com/riscv-non-isa/riscv-sbi-doc)
//!
//! ``` text
//! $ cargo new --bin app && cd $_
//!
//! $ # add this crate as a dependency
//! $ edit Cargo.toml && cat $_
//! [dependencies]
//! riscv-rt = "0.6.1"
//! panic-halt = "0.2.0"
//!
//! $ # memory layout of the device
//! $ edit memory.x && cat $_
//! MEMORY
//! {
//!   RAM : ORIGIN = 0x80000000, LENGTH = 16K
//!   FLASH : ORIGIN = 0x20000000, LENGTH = 16M
//! }
//!
//! REGION_ALIAS("REGION_TEXT", FLASH);
//! REGION_ALIAS("REGION_RODATA", FLASH);
//! REGION_ALIAS("REGION_DATA", RAM);
//! REGION_ALIAS("REGION_BSS", RAM);
//! REGION_ALIAS("REGION_HEAP", RAM);
//! REGION_ALIAS("REGION_STACK", RAM);
//!
//! $ edit src/main.rs && cat $_
//! ```
//!
//! ``` ignore,no_run
//! #![no_std]
//! #![no_main]
//!
//! extern crate panic_halt;
//!
//! use riscv_rt::entry;
//!
//! // use `main` as the entry point of this application
//! // `main` is not allowed to return
//! #[entry]
//! fn main() -> ! {
//!     // do something here
//!     loop { }
//! }
//! ```
//!
//! ``` text
//! $ mkdir .cargo && edit .cargo/config && cat $_
//! [target.riscv32imac-unknown-none-elf]
//! rustflags = [
//!   "-C", "link-arg=-Tmemory.x",
//!   "-C", "link-arg=-Tlink.x",
//! ]
//!
//! [build]
//! target = "riscv32imac-unknown-none-elf"
//! $ edit build.rs && cat $_
//! ```
//!
//! ``` ignore,no_run
//! use std::env;
//! use std::fs;
//! use std::path::PathBuf;
//!
//! fn main() {
//!     let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
//!
//!     // Put the linker script somewhere the linker can find it.
//!     fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();
//!     println!("cargo:rustc-link-search={}", out_dir.display());
//!     println!("cargo:rerun-if-changed=memory.x");
//!
//!     println!("cargo:rerun-if-changed=build.rs");
//! }
//! ```
//!
//! ``` text
//! $ cargo build
//!
//! $ riscv32-unknown-elf-objdump -Cd $(find target -name app) | head
//!
//! Disassembly of section .text:
//!
//! 20000000 <_start>:
//! 20000000:	800011b7          	lui	gp,0x80001
//! 20000004:	80018193          	addi	gp,gp,-2048 # 80000800 <_stack_start+0xffffc800>
//! 20000008:	80004137          	lui	sp,0x80004
//! ```
//!
//! # Symbol interfaces
//!
//! This crate makes heavy use of symbols, linker sections and linker scripts to
//! provide most of its functionality. Below are described the main symbol
//! interfaces.
//!
//! ## `memory.x`
//!
//! This file supplies the information about the device to the linker.
//!
//! ### `MEMORY`
//!
//! The main information that this file must provide is the memory layout of
//! the device in the form of the `MEMORY` command. The command is documented
//! [here][2], but at a minimum you'll want to create at least one memory region.
//!
//! [2]: https://sourceware.org/binutils/docs/ld/MEMORY.html
//!
//! To support different relocation models (RAM-only, FLASH+RAM) multiple regions are used:
//!
//! - `REGION_TEXT` - for `.init`, `.trap` and `.text` sections
//! - `REGION_RODATA` - for `.rodata` section and storing initial values for `.data` section
//! - `REGION_DATA` - for `.data` section
//! - `REGION_BSS` - for `.bss` section
//! - `REGION_HEAP` - for the heap area
//! - `REGION_STACK` - for hart stacks
//!
//! Specific aliases for these regions must be defined in `memory.x` file (see example below).
//!
//! ### `_stext`
//!
//! This symbol provides the loading address of `.text` section. This value can be changed
//! to override the loading address of the firmware (for example, in case of bootloader present).
//!
//! If omitted this symbol value will default to `ORIGIN(REGION_TEXT)`.
//!
//! ### `_stack_start`
//!
//! This symbol provides the address at which the call stack will be allocated.
//! The call stack grows downwards so this address is usually set to the highest
//! valid RAM address plus one (this *is* an invalid address but the processor
//! will decrement the stack pointer *before* using its value as an address).
//!
//! In case of multiple harts present, this address defines the initial stack pointer for hart 0.
//! Stack pointer for hart `N` is calculated as  `_stack_start - N * _hart_stack_size`.
//!
//! If omitted this symbol value will default to `ORIGIN(REGION_STACK) + LENGTH(REGION_STACK)`.
//!
//! #### Example
//!
//! Allocating the call stack on a different RAM region.
//!
//! ``` text
//! MEMORY
//! {
//!   L2_LIM : ORIGIN = 0x08000000, LENGTH = 1M
//!   RAM : ORIGIN = 0x80000000, LENGTH = 16K
//!   FLASH : ORIGIN = 0x20000000, LENGTH = 16M
//! }
//!
//! REGION_ALIAS("REGION_TEXT", FLASH);
//! REGION_ALIAS("REGION_RODATA", FLASH);
//! REGION_ALIAS("REGION_DATA", RAM);
//! REGION_ALIAS("REGION_BSS", RAM);
//! REGION_ALIAS("REGION_HEAP", RAM);
//! REGION_ALIAS("REGION_STACK", L2_LIM);
//!
//! _stack_start = ORIGIN(L2_LIM) + LENGTH(L2_LIM);
//! ```
//!
//! ### `_max_hart_id`
//!
//! This symbol defines the maximum hart id supported. All harts with id
//! greater than `_max_hart_id` will be redirected to `abort()`.
//!
//! This symbol is supposed to be redefined in platform support crates for
//! multi-core targets.
//!
//! If omitted this symbol value will default to 0 (single core).
//!
//! ### `_hart_stack_size`
//!
//! This symbol defines stack area size for *one* hart.
//!
//! If omitted this symbol value will default to 2K.
//!
//! ### `_heap_size`
//!
//! This symbol provides the size of a heap region. The default value is 0. You can set `_heap_size`
//! to a non-zero value if you are planning to use heap allocations.
//!
//! ### `_sheap`
//!
//! This symbol is located in RAM right after the `.bss` and `.data` sections.
//! You can use the address of this symbol as the start address of a heap
//! region. This symbol is 4 byte aligned so that address will be a multiple of 4.
//!
//! #### Example
//!
//! ``` no_run
//! extern crate some_allocator;
//!
//! extern "C" {
//!     static _sheap: u8;
//!     static _heap_size: u8;
//! }
//!
//! fn main() {
//!     unsafe {
//!         let heap_bottom = &_sheap as *const u8 as usize;
//!         let heap_size = &_heap_size as *const u8 as usize;
//!         some_allocator::initialize(heap_bottom, heap_size);
//!     }
//! }
//! ```
//!
//! ### `_mp_hook`
//!
//! This function is called from all the harts and must return true only for one hart,
//! which will perform memory initialization. For other harts it must return false
//! and implement wake-up in platform-dependent way (e.g. after waiting for a user interrupt).
//! The parameter `hartid` specifies the hartid of the caller.
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "_mp_hook"]
//! pub extern "Rust" fn mp_hook(hartid: usize) -> bool {
//!    // ...
//! }
//! ```
//!
//! Default implementation of this function wakes hart 0 and busy-loops all the other harts.
//!
//! ### `ExceptionHandler`
//!
//! This function is called when exception is occured. The exception reason can be decoded from the
//! `mcause`/`scause` register.
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "ExceptionHandler"]
//! fn custom_exception_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
//!     // ...
//! }
//! ```
//! or
//! ``` no_run
//! #[no_mangle]
//! fn ExceptionHandler(trap_frame: &riscv_rt::TrapFrame) -> ! {
//!     // ...
//! }
//! ```
//!
//! Default implementation of this function stucks in a busy-loop.
//!
//!
//! ### Core interrupt handlers
//!
//! This functions are called when corresponding interrupt is occured.
//! You can define an interrupt handler with one of the following names:
//! * `UserSoft`
//! * `SupervisorSoft`
//! * `MachineSoft`
//! * `UserTimer`
//! * `SupervisorTimer`
//! * `MachineTimer`
//! * `UserExternal`
//! * `SupervisorExternal`
//! * `MachineExternal`
//!
//! For example:
//! ``` no_run
//! #[export_name = "MachineTimer"]
//! fn custom_timer_handler() {
//!     // ...
//! }
//! ```
//! or
//! ``` no_run
//! #[no_mangle]
//! fn MachineTimer() {
//!     // ...
//! }
//! ```
//!
//! If interrupt handler is not explicitly defined, `DefaultHandler` is called.
//!
//! ### `DefaultHandler`
//!
//! This function is called when interrupt without defined interrupt handler is occured.
//! The interrupt reason can be decoded from the `mcause`/`scause` register.
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "DefaultHandler"]
//! fn custom_interrupt_handler() {
//!     // ...
//! }
//! ```
//! or
//! ``` no_run
//! #[no_mangle]
//! fn DefaultHandler() {
//!     // ...
//! }
//! ```
//!
//! Default implementation of this function stucks in a busy-loop.
//!
//! # Features
//!
//! ## `s-mode`
//!
//! The supervisor mode feature (`s-mode`) can be activated via [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
//!
//! For example:
//! ``` text
//! [dependencies]
//! riscv-rt = {features=["s-mode"]}
//! ```
//! Internally, riscv-rt uses different versions of precompiled static libraries
//! for (i) machine mode and (ii) supervisor mode. If the `s-mode` feature was activated,
//! the build script selects the s-mode library. While most registers/instructions have variants for
//! both `mcause` and `scause`, the `mhartid` hardware thread register is not available in supervisor
//! mode. Instead, the hartid is passed as parameter by a bootstrapping firmware (i.e., SBI).
//!
//! Use case: QEMU supports [OpenSBI](https://github.com/riscv-software-src/opensbi) as default firmware.
//! Using the SBI requires riscv-rt to be run in supervisor mode instead of machine mode.
//! ``` text
//! APP_BINARY=$(find target -name app)
//! sudo qemu-system-riscv64 -m 2G -nographic -machine virt -kernel $APP_BINARY
//! ```
//! It requires the memory layout to be non-overlapping, like
//! ``` text
//! MEMORY
//! {
//!   RAM : ORIGIN = 0x80200000, LENGTH = 0x8000000
//!   FLASH : ORIGIN = 0x20000000, LENGTH = 16M
//! }
//! ```

// NOTE: Adapted from cortex-m/src/lib.rs
#![no_std]
#![deny(missing_docs)]

use core::arch::global_asm;

#[cfg(feature = "clic")]
use riscv_clic as riscv_crate;

#[cfg(not(feature = "clic"))]
use ::riscv as riscv_crate;


#[cfg(feature = "s-mode")]
use riscv_crate::register::{scause as xcause, stvec as xtvec, stvec::TrapMode as xTrapMode};

#[cfg(not(feature = "s-mode"))]
use riscv_crate::register::{mcause as xcause, mhartid, mtvec as xtvec, mtvec::TrapMode as xTrapMode};

// TODO: enable this for s-mode
#[cfg(feature = "clic")]
use riscv_crate::register::{mtvt as xtvt, mtvec::SubMode as xSubMode};


pub use riscv_rt_macros::{entry, pre_init, interrupt_handler};

#[export_name = "error: riscv-rt appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;
}

/// Rust entry point (_start_rust)
///
/// Zeros bss section, initializes data section and calls main. This function
/// never returns.
#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub unsafe extern "C" fn start_rust(a0: usize, a1: usize, a2: usize) -> ! {
    #[rustfmt::skip]
    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main(a0: usize, a1: usize, a2: usize) -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();

        fn _setup_interrupts();

        fn _mp_hook(hartid: usize) -> bool;
    }

    // sbi passes hartid as first parameter (a0)
    #[cfg(feature = "s-mode")]
    let hartid = a0;
    #[cfg(not(feature = "s-mode"))]
    let hartid = mhartid::read();

    if _mp_hook(hartid) {
        __pre_init();

        r0::zero_bss(&mut _sbss, &mut _ebss);
        r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    }

    // TODO: Enable FPU when available

    _setup_interrupts();

    main(a0, a1, a2);
}

/// Registers saved in trap handler
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
}

/// Trap entry point rust (_start_trap_rust)
///
/// `scause`/`mcause` is read to determine the cause of the trap. XLEN-1 bit indicates
/// if it's an interrupt or an exception. The result is examined and ExceptionHandler
/// or one of the core interrupt handlers is called.
#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust"]
pub extern "C" fn start_trap_rust(trap_frame: *const TrapFrame) {
    extern "C" {
        fn ExceptionHandler(trap_frame: &TrapFrame);
        fn DefaultHandler();
    }

    unsafe {
        let cause = xcause::read();

        if cause.is_exception() {
            ExceptionHandler(&*trap_frame)
        } else {
            #[cfg(not(feature = "clic"))]
            if cause.code() < __INTERRUPTS.len() {
                let h = &__INTERRUPTS[cause.code()];
                if h.reserved == 0 {
                    DefaultHandler();
                } else {
                    (h.handler)();
                }
            } else {
                DefaultHandler();
            }
            #[cfg(feature = "clic")]
            DefaultHandler();
        }
    }
}

#[doc(hidden)]
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub fn DefaultExceptionHandler(trap_frame: &TrapFrame) -> ! {
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

#[doc(hidden)]
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub fn DefaultInterruptHandler() {
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

/* Interrupts */
#[cfg(not(feature = "clic"))]
#[doc(hidden)]
pub enum Interrupt {
    UserSoft,
    SupervisorSoft,
    MachineSoft,
    UserTimer,
    SupervisorTimer,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    MachineExternal,
}

#[cfg(not(feature = "clic"))]
pub use self::Interrupt as interrupt;

#[cfg(not(feature = "clic"))]
extern "C" {
    fn UserSoft();
    fn SupervisorSoft();
    fn MachineSoft();
    fn UserTimer();
    fn SupervisorTimer();
    fn MachineTimer();
    fn UserExternal();
    fn SupervisorExternal();
    fn MachineExternal();
}

#[cfg(not(feature = "clic"))]
#[doc(hidden)]
pub union Vector {
    pub handler: unsafe extern "C" fn(),
    pub reserved: usize,
}

#[cfg(not(feature = "clic"))]
#[doc(hidden)]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 12] = [
    Vector { handler: UserSoft },
    Vector {
        handler: SupervisorSoft,
    },
    Vector { reserved: 0 },
    Vector {
        handler: MachineSoft,
    },
    Vector { handler: UserTimer },
    Vector {
        handler: SupervisorTimer,
    },
    Vector { reserved: 0 },
    Vector {
        handler: MachineTimer,
    },
    Vector {
        handler: UserExternal,
    },
    Vector {
        handler: SupervisorExternal,
    },
    Vector { reserved: 0 },
    Vector {
        handler: MachineExternal,
    },
];

#[doc(hidden)]
#[no_mangle]
#[rustfmt::skip]
pub unsafe extern "Rust" fn default_pre_init() {}

#[doc(hidden)]
#[no_mangle]
#[rustfmt::skip]
pub extern "Rust" fn default_mp_hook(hartid: usize) -> bool {
    match hartid {
        0 => true,
        _ => loop {
            unsafe { riscv::asm::wfi() }
        },
    }
}

/// Default implementation of `_setup_interrupts` for CLINT that sets `mtvec`/`stvec` to a trap handler address.
#[doc(hidden)]
#[no_mangle]
#[rustfmt::skip]
#[cfg(not(feature = "clic"))]
pub unsafe extern "Rust" fn default_setup_interrupts() {
    {
        extern "C" {
            fn _start_trap();
        }   
        xtvec::write(_start_trap as usize, xTrapMode::Direct);
    }
}

/// Default implementation of `_setup_interrupts` for CLIC that
/// 
#[doc(hidden)]
#[no_mangle]
#[rustfmt::skip]
#[cfg(feature = "clic")]
pub unsafe extern "Rust" fn default_setup_interrupts() {

    {
        extern "C" {
            fn _start_trap();
            fn _nxti_trap_handler();
        }   

        extern {
            static interrupt_vector: usize;
        }

        if cfg!(feature = "nxti") {
            // _nxti_trap_handler handles context saving and executes cycles through all pending interrupts via the nxti feature
            xtvec::write(_nxti_trap_handler as usize, xSubMode::Default, xTrapMode::Clic);       
        }
        else
        {
            // _start_trap handles all non vectored interrupts and all exceptions
            xtvec::write(_start_trap as usize, xSubMode::Default, xTrapMode::Clic);       
        }

        let interrupt_vector_ptr:*const usize = &interrupt_vector;
        xtvt::write_addr(interrupt_vector_ptr as usize);
    }
}

#[cfg(all(feature = "clic", feature = "nxti"))]
global_asm!("
/* NXTI interrupt handler */
.section .text.nxti_trap_handler
.global _nxti_trap_handler
_nxti_trap_handler:
/* store context */
addi sp, sp, -(4 * 32)
sw ra, 0(sp)
sw t0, 4(sp)
sw t1, 8(sp)
sw t2, 12(sp)
sw a0, 16(sp)
sw a1, 20(sp)
sw a2, 24(sp)
sw a3, 28(sp)
sw a4, 32(sp)
sw a5, 36(sp)
sw a6, 40(sp)
sw a7, 44(sp)
sw t3, 48(sp)
sw t4, 52(sp)
sw t5, 56(sp)
sw t6, 60(sp)
csrr t0, mcause
csrr t1, mepc
sw t0, 64(sp)
sw t1, 68(sp)

/* read out the address of the mtvt entry of the next pending interrupt */
/* enables interrupts, and clears the pending bit of the found interrupt */
1:
csrrsi t0, 0x345, 8


/* if no interrupt is pending, the received addr (t0) will be 0 */
beqz t0, 2f

/* jump to interrupt vector table */
jalr t0

/* repeat until no more interrupts are pending */
j 1b

2:

csrci mstatus, 8 /* disable global interrupts*/

/* load context */
lw t0, 64(sp)
lw t1, 68(sp)
csrw mcause, t0
csrw mepc, t1
lw ra, 0(sp)
lw t0, 4(sp)
lw t1, 8(sp)
lw t2, 12(sp)
lw a0, 16(sp)
lw a1, 20(sp)
lw a2, 24(sp)
lw a3, 28(sp)
lw a4, 32(sp)
lw a5, 36(sp)
lw a6, 40(sp)
lw a7, 44(sp)
lw t3, 48(sp)
lw t4, 52(sp)
lw t5, 56(sp)
lw t6, 60(sp)
addi sp, sp, (4 * 32)

/* return to previous code before context save */
mret
");

#[cfg(feature = "clic")]
global_asm!("
.section .text.interrupt_vector
.option norvc
.global interrupt_vector
interrupt_vector:
j int_0
j int_1
j int_2
j int_3
j int_4
j int_5
j int_6
j int_7
j int_8
j int_9
j int_10
j int_11
j int_12
j int_13
j int_14
j int_15
j int_16
j int_17
j int_18
j int_19
j int_20
j int_21
j int_22
j int_23
j int_24
j int_25
j int_26
j int_27
j int_28
j int_29
j int_30
j int_31
j int_32
j int_33
j int_34
j int_35
j int_36
j int_37
j int_38
j int_39
j int_40
j int_41
j int_42
j int_43
j int_44
j int_45
j int_46
j int_47
j int_48
j int_49
j int_50
j int_51
j int_52
j int_53
j int_54
j int_55
j int_56
j int_57
j int_58
j int_59
j int_60
j int_61
j int_62
j int_63
j int_64
j int_65
j int_66
j int_67
j int_68
j int_69
j int_70
j int_71
j int_72
j int_73
j int_74
j int_75
j int_76
j int_77
j int_78
j int_79
j int_80
j int_81
j int_82
j int_83
j int_84
j int_85
j int_86
j int_87
j int_88
j int_89
j int_90
j int_91
j int_92
j int_93
j int_94
j int_95
j int_96
j int_97
j int_98
j int_99
j int_100
j int_101
j int_102
j int_103
j int_104
j int_105
j int_106
j int_107
j int_108
j int_109
j int_110
j int_111
j int_112
j int_113
j int_114
j int_115
j int_116
j int_117
j int_118
j int_119
j int_120
j int_121
j int_122
j int_123
j int_124
j int_125
j int_126
j int_127
j int_128
j int_129
j int_130
j int_131
j int_132
j int_133
j int_134
j int_135
j int_136
j int_137
j int_138
j int_139
j int_140
j int_141
j int_142
j int_143
j int_144
j int_145
j int_146
j int_147
j int_148
j int_149
j int_150
j int_151
j int_152
j int_153
j int_154
j int_155
j int_156
j int_157
j int_158
j int_159
j int_160
j int_161
j int_162
j int_163
j int_164
j int_165
j int_166
j int_167
j int_168
j int_169
j int_170
j int_171
j int_172
j int_173
j int_174
j int_175
j int_176
j int_177
j int_178
j int_179
j int_180
j int_181
j int_182
j int_183
j int_184
j int_185
j int_186
j int_187
j int_188
j int_189
j int_190
j int_191
j int_192
j int_193
j int_194
j int_195
j int_196
j int_197
j int_198
j int_199
j int_200
j int_201
j int_202
j int_203
j int_204
j int_205
j int_206
j int_207
j int_208
j int_209
j int_210
j int_211
j int_212
j int_213
j int_214
j int_215
j int_216
j int_217
j int_218
j int_219
j int_220
j int_221
j int_222
j int_223
j int_224
j int_225
j int_226
j int_227
j int_228
j int_229
j int_230
j int_231
j int_232
j int_233
j int_234
j int_235
j int_236
j int_237
j int_238
j int_239
j int_240
j int_241
j int_242
j int_243
j int_244
j int_245
j int_246
j int_247
j int_248
j int_249
j int_250
j int_251
j int_252
j int_253
j int_254
j int_255
j int_256
j int_257
j int_258
j int_259
j int_260
j int_261
j int_262
j int_263
j int_264
");
