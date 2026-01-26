#include "rvbl/c/string.h"
#include "rvbl/test/rvbl_test.h"

int main(void)
{
#define SIZE 32
    const int value = 0xBA;
    rvbl_uint8_t buffer[SIZE];
    size_t i;

    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();

    ASSERT_EQ(memset(buffer, value, SIZE), buffer);

    for (i = 0; i < SIZE; ++i) {
        ASSERT_EQ(buffer[i], value);
    }

    PASS();

    return 0;
}
