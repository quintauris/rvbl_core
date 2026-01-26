#include "rvbl/core/rvbl_abi.h"
#include "rvbl/core/rvbl_types.h"
#include "rvbl/machine/rvbl_machine.h"
#include "rvbl/test/rvbl_test.h"

__attribute__((naked)) void global_interrupt_handler(void)
{
    __asm__ volatile("csrw mscratch, a0\n\t"
                     "csrr a0, mepc\n\t"
                     "addi a0, a0, 4\n\t"
                     "csrw mepc, a0\n\t"
                     "csrr a0, mscratch\n\t"
                     "add a0, a0, a1\n\t"
                     "add a0, a0, a2\n\t"
                     "add a0, a0, a3\n\t"
                     "mret\n\t");
}

int main(void)
{
    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();
    rvbl_mtvec_write((rvbl_uint32_t)&global_interrupt_handler);
    ASSERT_EQ(rvbl_system_call_3(23, 1, 2, 3), 29);
    PASS();

    return 0;
}
