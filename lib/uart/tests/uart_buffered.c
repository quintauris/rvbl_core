/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/test/rvbl_test.h"
#include "rvbl/type/rvbl_types.h"

#include "rvbl/uart/rvbl_uart.h"

static rvbl_bool_t uart_initialized = rvbl_false;
static rvbl_uword_t uart_bytes_putc = 0, uart_bytes_write = 0;
static char buffer[256];

void test_uart_init(const void *self)
{
    (void)self;
    uart_initialized = rvbl_true;
}

void test_uart_putc(const void *self, const int c, void *const p)
{
    (void)self;
    (void)c;
    (void)p;
    uart_bytes_putc++;
}

void test_uart_write(const void *self, const void *buffer, const rvbl_uword_t count, void *p)
{
    (void)self;
    (void)buffer;
    (void)p;
    uart_bytes_write += count;
}

int main(void)
{
    rvbl_uart test_uart;
    rvbl_buffered_uart buffered_uart;
    rvbl_uword_t i;

    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();

    test_uart.init = (rvbl_uart_init)test_uart_init;
    test_uart.putc = (rvbl_uart_putc)test_uart_putc;
    test_uart.write = NULL;

    buffered_uart.uart_interface = &test_uart;
    buffered_uart.uart_instance = NULL;
    buffered_uart.buffer = buffer;
    buffered_uart.buffer_size = sizeof(buffer);

    rvbl_buffered_uart_init(&buffered_uart);
    ASSERT_EQ(buffered_uart.buffer_position, 0);
    ASSERT(uart_initialized);

    rvbl_buffered_uart_putc(&buffered_uart, 'a', NULL);
    ASSERT_EQ(buffer[0], 'a');
    ASSERT_EQ(buffered_uart.buffer_position, 1);
    ASSERT_EQ(uart_bytes_putc, 0);

    rvbl_buffered_uart_flush(&buffered_uart, NULL);
    ASSERT_EQ(buffered_uart.buffer_position, 0);
    ASSERT_EQ(uart_bytes_putc, 1);

    uart_bytes_putc = 0;

    for (i = 0; i < sizeof(buffer); ++i) {
        rvbl_buffered_uart_putc(&buffered_uart, 'a', NULL);
    }

    ASSERT_EQ(buffered_uart.buffer_position, sizeof(buffer));
    rvbl_buffered_uart_putc(&buffered_uart, 'a', NULL);
    ASSERT_EQ(buffered_uart.buffer_position, sizeof(buffer));
    ASSERT_EQ(uart_bytes_putc, 0);

    test_uart.write = (rvbl_uart_write)test_uart_write;
    rvbl_buffered_uart_flush(&buffered_uart, NULL);
    ASSERT_EQ(buffered_uart.buffer_position, 0);
    ASSERT_EQ(uart_bytes_write, sizeof(buffer));

    PASS();
    return 0;
}
