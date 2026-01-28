#include "rvbl/uart/rvbl_uart.h"
#include "rvbl/core/rvbl_types.h"
#include "rvbl/uart/rvbl_uart_ns16550.h"

#include "rvbl/machine/rvbl_uart_ns16550.h"

void rvbl_uart_ns16550_init(const struct rvbl_uart_ns16550_t *const instance) {}

void rvbl_uart_ns16550_putc(const struct rvbl_uart_ns16550_t *const instance, const int c, void *p)
{
    rvbl_uart_ns16550_data_write(instance, c);
}

int rvbl_uart_ns16550_getc(const struct rvbl_uart_ns16550_t *const instance, void *p)
{
    return rvbl_uart_ns16550_data_read(instance);
}

rvbl_bool_t rvbl_uart_ns16550_rx_ready(const struct rvbl_uart_ns16550_t *const instance, void *p)
{
    return rvbl_uart_ns16550_line_status_data_ready(instance);
}

rvbl_bool_t rvbl_uart_ns16550_tx_ready(const struct rvbl_uart_ns16550_t *const instance, void *p)
{
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
