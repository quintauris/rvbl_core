/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/uart/rvbl_uart.h"
#include "rvbl/hardware/rvbl_hardware.h"
#include "rvbl/type/rvbl_types.h"
#include "rvbl/uart/rvbl_uart_ns16550.h"

#include "rvbl/machine/rvbl_uart_ns16550.h"

void rvbl_uart_ns16550_init(const struct rvbl_uart_ns16550_t *const instance) { (void)instance; }

void rvbl_uart_ns16550_putc(const struct rvbl_uart_ns16550_t *const instance, const int c, void *p)
{
    (void)p;
    RVBL_REGISTER_WRITE(uart_ns16550, registers, instance, data, c);
}

int rvbl_uart_ns16550_getc(const struct rvbl_uart_ns16550_t *const instance, void *p)
{
    (void)p;
    return RVBL_REGISTER_READ(uart_ns16550, registers, instance, data);
}

rvbl_bool_t rvbl_uart_ns16550_rx_ready(const struct rvbl_uart_ns16550_t *const instance, void *p)
{
    (void)p;
    return RVBL_REGISTER_FIELD_READ(uart_ns16550, registers, instance, line_status, data_ready);
}

rvbl_bool_t rvbl_uart_ns16550_tx_ready(const struct rvbl_uart_ns16550_t *const *instance, void *p)
{
    (void)p;
    (void)instance;
    return rvbl_true;
}

const rvbl_uart rvbl_uart_ns16550 = {
    .init = (rvbl_uart_init)rvbl_uart_ns16550_init,
    .rx_ready = (rvbl_uart_rx_ready)rvbl_uart_ns16550_rx_ready,
    .getc = (rvbl_uart_getc)rvbl_uart_ns16550_getc,
    .tx_ready = (rvbl_uart_tx_ready)rvbl_uart_ns16550_tx_ready,
    .putc = (rvbl_uart_putc)rvbl_uart_ns16550_putc,
    .write = NULL
};
