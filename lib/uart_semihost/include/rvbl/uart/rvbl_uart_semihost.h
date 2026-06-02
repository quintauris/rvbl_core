#ifndef RVBL_UART_SEMIHOST_H
#define RVBL_UART_SEMIHOST_H

#include "rvbl/uart/rvbl_uart.h"

/// == Global `rvbl_uart_semihost`
///
/// Set of function pointers implementing access to a semihost UART device.
extern const rvbl_uart rvbl_uart_semihost;

/// == Type `rvbl_uart_semihost_t`
///
/// Device control block for semihost UART instances.
struct rvbl_uart_semihost_t
{
    rvbl_uword_t handle;
};

#endif
