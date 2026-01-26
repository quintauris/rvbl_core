.global _start
.global hang

.extern rvbl_boot
.extern general_purpose_stack_top
.extern main
.extern rvbl_boot_barrier

.set BARRIER_MAGIC, 1450

.section .text.start

_start:
    ; /* Only HART0 performs the boot process */
    csrr a0, mhartid
    bnez a0, wait
    la sp, general_purpose_stack_top
    call rvbl_boot
    addi a1, zero, BARRIER_MAGIC
    la a2, rvbl_boot_barrier
    sw a1, 0(a2)
    fence
    j boot_done
wait:
    ; /* Other HARTs wait until rvbl_boot_barrier == BARRIER_MAGIC */
    addi a1, zero, BARRIER_MAGIC
    la a2, rvbl_boot_barrier
wait_loop:
    fence
    lw a0, 0(a2)
    bne a0, a1, wait_loop
boot_done:
    ; /* 1K stack for each HART */
    csrr t0, mhartid
    addi t1, zero, -1024
    mul t1, t0, t1
    la sp, general_purpose_stack_top
    add sp, sp, t1
    call main
    j hang

_c_init:
    ret

hang:
    wfi
    j hang

.end
