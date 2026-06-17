/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/c/stdlib.h"
#include "rvbl/test/rvbl_test.h"

// Some linkers might fail if this symbol is not defined, even if it is not used
rvbl_alloc_allocator rvbl_c_stdlib_allocator;

int main(void)
{
    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();

    ASSERT_EQ(abs(-23), 23);
    ASSERT_EQ(abs(23), 23);
    ASSERT_EQ(abs(0), 0);

    ASSERT_EQ(abs(-23l), 23l);
    ASSERT_EQ(abs(23l), 23l);
    ASSERT_EQ(abs(0l), 0l);

    PASS();

    return 0;
}
