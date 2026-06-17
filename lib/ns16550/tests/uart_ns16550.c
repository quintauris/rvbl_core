/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/machine/rvbl_machine.h"
#include "rvbl/test/rvbl_test.h"
#include "rvbl/uart/rvbl_uart_ns16550.h"

int main(void)
{
    const char *message = "PASS\n";

    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();

    rvbl_uart_ns16550.init(&rvbl_uart_ns16550_instance_default);

    for (const char *i = message; *i != '\0'; ++i) {
        while (!rvbl_uart_ns16550.tx_ready(&rvbl_uart_ns16550_instance_default, NULL)) {
        }
        rvbl_uart_ns16550.putc(&rvbl_uart_ns16550_instance_default, *i, NULL);
    }

    rvbl_semihost_exit(rvbl_semihost_exit_code_success);

    return 0;
}
