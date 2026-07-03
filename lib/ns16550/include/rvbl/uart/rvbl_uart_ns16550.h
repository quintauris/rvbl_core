/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = NS16550 Device Library
/// Quintauris GmbH
///
/// Implements NS16550 UART interface.

#ifndef RVBL_UART_NS_16550_H
#define RVBL_UART_NS_16550_H

/// == <rvbl_uart_ns16550.h>

#include "rvbl/uart/rvbl_uart.h"

/// === Global `rvbl_uart_ns16550`
///
/// Set of function pointers implementing access to NS16550 UART device.
extern const rvbl_uart rvbl_uart_ns16550;

#endif
