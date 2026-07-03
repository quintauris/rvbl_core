/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/uart/rvbl_uart.h"
#include "rvbl/type/rvbl_types.h"

void rvbl_buffered_uart_init(rvbl_buffered_uart *const self)
{
    self->uart_interface->init(self->uart_instance);
    self->buffer_position = 0;
}

void rvbl_buffered_uart_putc(rvbl_buffered_uart *const self, int c, void *p)
{
    (void)p;
    if (self->buffer_position < self->buffer_size) {
        ((rvbl_uint8_t *)self->buffer)[self->buffer_position++] = c;
    }
}

void rvbl_buffered_uart_flush(rvbl_buffered_uart *const self, void *p)
{
    const char *buffer = self->buffer;
    rvbl_uword_t i = 0;

    if (self->uart_interface->write) {
        self->uart_interface->write(self->uart_instance, buffer, self->buffer_position, p);
    } else {
        for (; i < self->buffer_position; ++i) {
            self->uart_interface->putc(self->uart_instance, buffer[i], p);
        }
    }

    self->buffer_position = 0;
}
