/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/test/rvbl_test.h"

#include "rvbl/uart/rvbl_uart_semihost.h"

int main(void)
{
    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();
    PASS();
    return 0;
}
