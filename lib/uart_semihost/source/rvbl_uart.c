/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/uart/rvbl_uart_semihost.h"

#include "rvbl/semihost/rvbl_semihost.h"

void rvbl_uart_semihost_init(struct rvbl_uart_semihost_t *const instance)
{
    instance->handle = rvbl_semihost_output_stream_open(":tt", rvbl_semihost_file_mode_write_plus);
}

void rvbl_uart_semihost_putc(
    const struct rvbl_uart_semihost_t *const instance, const int c, void *p
)
{
    (void)p;
    rvbl_semihost_output_stream_write(instance->handle, (const rvbl_uint8_t *)&c, 1);
}

void rvbl_uart_semihost_write(
    const struct rvbl_uart_semihost_t *const instance,
    const void *const buffer,
    const rvbl_uword_t size,
    void *p
)
{
    (void)p;
    rvbl_semihost_output_stream_write(instance->handle, (const rvbl_uint8_t *)buffer, size);
}

int rvbl_uart_semihost_getc(const struct rvbl_uart_semihost_t *const instance, void *p)
{
    (void)instance;
    (void)p;
    return 0;
}

rvbl_bool_t rvbl_uart_semihost_rx_ready(const struct rvbl_uart_semihost_t *const instance, void *p)
{
    (void)instance;
    (void)p;
    return rvbl_false;
}

rvbl_bool_t rvbl_uart_semihost_tx_ready(const struct rvbl_uart_semihost_t *const instance, void *p)
{
    (void)instance;
    (void)p;
    return rvbl_true;
}

const rvbl_uart rvbl_uart_semihost = {
    .init = (rvbl_uart_init)rvbl_uart_semihost_init,
    .rx_ready = (rvbl_uart_rx_ready)rvbl_uart_semihost_rx_ready,
    .getc = (rvbl_uart_getc)rvbl_uart_semihost_getc,
    .tx_ready = (rvbl_uart_tx_ready)rvbl_uart_semihost_tx_ready,
    .putc = (rvbl_uart_putc)rvbl_uart_semihost_putc,
    .write = (rvbl_uart_write)rvbl_uart_semihost_write
};
