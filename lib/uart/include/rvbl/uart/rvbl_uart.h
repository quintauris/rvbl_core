/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = UART Device Library
/// Quintauris GmbH
///
/// Provides generic interfaces to UART devices.

#ifndef RVBL_UART_H
#define RVBL_UART_H

/// == <rvbl_uart.h>

#include "rvbl/type/rvbl_types.h"

/// === Type `rvbl_uart_init`
///
/// Function pointer to an UART device initialization function.
typedef void (*rvbl_uart_init)(const void *);

/// === Type `rvbl_uart_putc`
///
/// Function pointer to an UART device char-outputting function.
typedef void (*rvbl_uart_putc)(const void *, const int c, void *);

/// === Type `rvbl_uart_getc`
///
/// Function pointer to an UART device char-inputting function.
typedef int (*rvbl_uart_getc)(const void *, void *);

/// === Type `rvbl_uart_rx_ready`
///
/// Function pointer to an UART device RX-readyness checking function.
typedef rvbl_bool_t (*rvbl_uart_rx_ready)(const void *, void *);

/// === Type `rvbl_uart_write`
///
/// Function pointer to an UART device synchronous write function.
typedef void (*rvbl_uart_write)(const void *, const void *, const rvbl_uword_t, void *);

/// === Type `rvbl_uart_tx_ready`
///
/// Function pointer to an UART device synchronous write function.
typedef rvbl_bool_t (*rvbl_uart_tx_ready)(const void *, const void *);

/// === Type `rvbl_uart`
///
/// Set of function pointers implementing a particular UART device.
typedef struct rvbl_uart
{
    /// `rvbl_uart_init init`:: Initialization function.
    rvbl_uart_init init;

    /// `rvbl_uart_rx_ready rx_ready`:: RX-ready function.
    rvbl_uart_rx_ready rx_ready;

    /// `rvbl_uart_getc getc`:: Character input function.
    rvbl_uart_getc getc;

    /// `rvbl_uart_tx_ready tx_ready`:: TX-ready function.
    rvbl_uart_tx_ready tx_ready;

    /// `rvbl_uart_putc putc`:: Character output function.
    rvbl_uart_putc putc;

    /// `rvbl_uart_write write`:: Synchronous write function.
    rvbl_uart_write write;

} rvbl_uart;

/// === Type `rvbl_buffered_uart`
///
/// Set of UART/buffer pointers implementing buffered UART access.
typedef struct rvbl_buffered_uart
{
    /// `rvbl_uart* uart_interface`:: Pointer to underlying UART interface.
    rvbl_uart *uart_interface;

    /// `void* uart_instance`:: Pointer to underlying UART instance.
    void *uart_instance;

    /// `void* buffer`:: Buffer for buffered transmission. Set to NULL to disable
    /// buffering.
    void *buffer;

    /// `rvbl_uword_t buffer_size`:: Buffer for buffered transmission. Set to NULL to disable
    /// buffering.
    rvbl_uword_t buffer_size;

    /// `rvbl_uword_t buffer_position`:: Index of first byte available in buffer.
    rvbl_uword_t buffer_position;
} rvbl_buffered_uart;

/// === Function `rvbl_buffered_uart_init`
///
/// Initializes buffered UART device.
///
/// ==== Parameters
///
/// `rvbl_buffered_uart*`:: Device instance.
void rvbl_buffered_uart_init(rvbl_buffered_uart *self);

/// === Function `rvbl_buffered_uart_putc`
///
/// Synchronously writes a character to the buffered UART device.
///
/// ==== Parameters
///
/// `rvbl_buffered_uart*`:: Device instance.
/// `int`:: Character to write.
/// `void*`:: Pointer carrying extra information (optional).
void rvbl_buffered_uart_putc(rvbl_buffered_uart *self, int c, void *p);

/// === Function `rvbl_buffered_uart_flush`
///
/// Synchronously writes all buffered characters to the underlying UART device.
///
/// ==== Parameters
///
/// `rvbl_buffered_uart*`:: Device instance.
/// `void*`:: Pointer carrying extra information (optional).
void rvbl_buffered_uart_flush(rvbl_buffered_uart *self, void *p);

#endif
