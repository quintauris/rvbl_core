/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/alloc/rvbl_alloc.h"
#include "rvbl/test/rvbl_test.h"

int main(void)
{
    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();

    ASSERT_EQ(rvbl_alloc_align((void *)0x00000000, 4), (void *)0x00000000);
    ASSERT_EQ(rvbl_alloc_align((void *)0x00000000, 64), (void *)0x00000000);
    ASSERT_EQ(rvbl_alloc_align((void *)0x00000002, 4), (void *)0x00000004);
    ASSERT_EQ(rvbl_alloc_align((void *)0x8001af04, 4), (void *)0x8001af04);
    ASSERT_EQ(rvbl_alloc_align((void *)0x8001af03, 4), (void *)0x8001af04);
    ASSERT_EQ(rvbl_alloc_align((void *)0x8001af03, 64), (void *)0x8001Af40);

    PASS();

    return 0;
}
