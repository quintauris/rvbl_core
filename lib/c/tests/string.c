#include "rvbl/c/string.h"
#include "rvbl/test/rvbl_test.h"

int main(void)
{
    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();

    ASSERT_EQ(strlen(""), 0);
    ASSERT_EQ(strlen("foo"), 3);

    ASSERT_EQ(strcmp("", ""), 0);
    ASSERT(strcmp("abc", "") > 0);
    ASSERT(strcmp("", "abc") < 0);
    ASSERT(strcmp("abc", "abd") < 0);
    ASSERT(strcmp("abd", "abc") > 0);

    PASS();

    return 0;
}
