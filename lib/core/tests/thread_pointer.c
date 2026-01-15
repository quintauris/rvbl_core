#include "rvbl/core/rvbl_abi.h"
#include "rvbl/test/rvbl_test.h"

int main(void)
{
    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();

    rvbl_thread_pointer_register_write(NULL);
    ASSERT_EQ(rvbl_thread_pointer_register_read(), NULL);

    rvbl_thread_pointer_register_write((rvbl_pointer_t)0xDEADFACE);
    ASSERT_EQ(rvbl_thread_pointer_register_read(), (rvbl_pointer_t)0xDEADFACE);

    rvbl_thread_pointer_register_write((rvbl_pointer_t)0xFFFFFFFF);
    ASSERT_EQ(rvbl_thread_pointer_register_read(), (rvbl_pointer_t)0xFFFFFFFF);

    PASS();

    return 0;
}