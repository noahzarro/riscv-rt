/* entry point */
.section .text.start
.global _start
.type _start, @function

_start:
/* initialize global pointer */
// the norelax option fails at link time
//.option push
//.option norelax
1:	auipc gp, %pcrel_hi(__global_pointer$)
	addi  gp, gp, %pcrel_lo(1b)
//.option pop

        csrr a0, 0xf14 /* Cluster ID */
        andi a1, a0, 0x1f       /* Core ID */
        srli a0, a0, 5

#if defined(CONFIG_CLUSTER)
        li a2, 31 /* FC_Cluster ID */
        //bne a2, a0, cl_cluster_exec_loop
        .extern cluster_exec_loop
        bne a2, a0, cluster_exec_loop
#endif
/* initialize stack pointer */
	la sp, __stack_top

/* set vector table address */
	la a0, __vector_start
	or a0, a0, 3 /* enable clic vectored mode (hardcoded anyway for RI5CY) */
	csrw mtvec, a0
#ifdef CONFIG_CLIC
	csrw 0x307, a0 /* mtvt=0x307 */ /* enable vectored mode TODO: should be clic mode */
#endif

/* clear the bss segment */
	la t0, __bss_start
        la t1, __bss_end
1:
	sw zero,0(t0)
	addi t0, t0, 4
	bltu t0, t1, 1b

/* new-style constructors and destructors */
#if defined(__PULP_USE_LIBC)
	la a0, __libc_fini_array
	call atexit
	call __libc_init_array
#endif

/* main gets called in rust */
	jal zero, _start_rust

.size  _start, .-_start

.global _init
.type   _init, @function
.global _fini
.type   _fini, @function
_init:
_fini:
 /* These don't have to do anything since we use init_array/fini_array. Prevent
    missing symbol error */
	ret
.size  _init, .-_init
.size _fini, .-_fini

/* indicate expected heap size to linker script, otherwise fall back to linker
script default */
#ifdef __HEAP_SIZE
.global __heap_size
	.equ __heap_size, __HEAP_SIZE
#endif

/* indicate (main) stack size to linker script, otherwise fall back to
linker script default */
#ifdef __STACK_SIZE
.global __stack_size
	.equ __stack_size, __STACK_SIZE
#endif

/*
    Trap entry point (_start_trap)

    Saves caller saved registers ra, t0..6, a0..7, calls _start_trap_rust,
    restores caller saved registers and then returns.
*/
.balign 64
.section .trap, "ax"
.global default_start_trap


default_start_trap:
    addi sp, sp, -16*4

    sw ra, 0*4(sp)
    sw t0, 1*4(sp)
    sw t1, 2*4(sp)
    sw t2, 3*4(sp)
    sw t3, 4*4(sp)
    sw t4, 5*4(sp)
    sw t5, 6*4(sp)
    sw t6, 7*4(sp)
    sw a0, 8*4(sp)
    sw a1, 9*4(sp)
    sw a2, 10*4(sp)
    sw a3, 11*4(sp)
    sw a4, 12*4(sp)
    sw a5, 13*4(sp)
    sw a6, 14*4(sp)
    sw a7, 15*4(sp)

    add a0, sp, zero
    jal ra, _start_trap_rust

    lw ra, 0*4(sp)
    lw t0, 1*4(sp)
    lw t1, 2*4(sp)
    lw t2, 3*4(sp)
    lw t3, 4*4(sp)
    lw t4, 5*4(sp)
    lw t5, 6*4(sp)
    lw t6, 7*4(sp)
    lw a0, 8*4(sp)
    lw a1, 9*4(sp)
    lw a2, 10*4(sp)
    lw a3, 11*4(sp)
    lw a4, 12*4(sp)
    lw a5, 13*4(sp)
    lw a6, 14*4(sp)
    lw a7, 15*4(sp)

    addi sp, sp, 16*4
    mret

.section .text
.global default_setup_interrupts

default_setup_interrupts:
    // Set trap handler
    la t0, _start_trap
    andi t0, t0, 0xc0
    ori t0, t0, 0x3
    csrw mtvec, t0
    ret


.section .text.vectors
default_exception_handler:
	j default_exception_handler
software_handler:
	j software_handler
timer_handler:
	j timer_handler
external_handler:
	j external_handler
__no_irq_handler:
	j __no_irq_handler

.section .text.vectors
.option norvc
.balign 1024
.global __clic_vector_table
__clic_vector_table:
	j default_exception_handler
	j __no_irq_handler
	j __no_irq_handler
	j software_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j timer_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j external_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __no_irq_handler
	j __handle_clic_int

.global __handle_clic_int
.func
__handle_clic_int:
	addi sp, sp, -(4 * 16)
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

	la   t0, clic_isr_hook
	lw   t0, 0(t0) /* load function pointer */

	jalr t0 /* vector into hook */

	/* TODO: write to completion register */

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
	addi sp, sp, (4 * 16)
	mret


/* Make sure there is an abort when linking */
.globl abort
abort:
    j abort
