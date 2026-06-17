/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/c/string.h"
#include "rvbl/test/rvbl_test.h"

int main(void)
{
    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();
    char buffer[32];

    ASSERT_EQ(strlen(""), 0);
    ASSERT_EQ(strlen("foo"), 3);

    ASSERT_EQ(strcmp("", ""), 0);
    ASSERT(strcmp("abc", "") > 0);
    ASSERT(strcmp("", "abc") < 0);
    ASSERT(strcmp("abc", "abd") < 0);
    ASSERT(strcmp("abd", "abc") > 0);

    ASSERT(strcpy(buffer, "") == buffer);
    ASSERT_EQ(strlen(buffer), 0);
    ASSERT_EQ(strcmp(buffer, ""), 0);
    ASSERT(strcpy(buffer, "abc") == buffer);
    ASSERT_EQ(strlen(buffer), 3);
    ASSERT_EQ(strcmp(buffer, "abc"), 0);

    PASS();

    return 0;
}
