#include "rvbl/test/rvbl_test.h"

#include "rvbl/uart/rvbl_uart_semihost.h"

int main(void)
{
    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();
    PASS();
    return 0;
}
