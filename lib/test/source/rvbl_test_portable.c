/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/test/rvbl_test.h"

#define NANOPRINTF_IMPLEMENTATION
#define NANOPRINTF_USE_FLOAT_FORMAT_SPECIFIERS 0
#define NANOPRINTF_USE_FIELD_WIDTH_FORMAT_SPECIFIERS 1
#define NANOPRINTF_USE_PRECISION_FORMAT_SPECIFIERS 0
#ifdef __TASKING__
#define NANOPRINTF_USE_LARGE_FORMAT_SPECIFIERS 0
#else
#define NANOPRINTF_USE_LARGE_FORMAT_SPECIFIERS 1
#endif
#define NANOPRINTF_USE_BINARY_FORMAT_SPECIFIERS 0
#define NANOPRINTF_USE_WRITEBACK_FORMAT_SPECIFIERS 0
#define NANOPRINTF_USE_SMALL_FORMAT_SPECIFIERS 0
#define NANOPRINTF_VISIBILITY_STATIC 1
#include <nanoprintf.h>

#include "rvbl/alloc/rvbl_alloc_bump.h"
#include "rvbl/c/string.h"
#include "rvbl/machine/rvbl_machine.h"

#define heap_size 1024
__attribute__((aligned(4))) static rvbl_uint8_t heap[heap_size];
static rvbl_alloc_area heap_area = {.base = heap, .size = heap_size};

rvbl_alloc_allocator rvbl_test_allocator = {
    .implementation = &rvbl_alloc_bump_allocator, .area_count = 1, .areas = &heap_area
};

void rvbl_test_shutdown(const TestResult result)
{
    switch (result) {
    case TestResult_Pass:
        rvbl_test_log("PASS");
        break;
    case TestResult_Fail:
        rvbl_test_log("FAIL");
        break;
    case TestResult_Skip:
        rvbl_test_log("SKIP");
        break;
    default:
        rvbl_test_log("USER(%d)", (int)result);
        break;
    }

    rvbl_semihost_exit((rvbl_semihost_exit_code)result);
}

extern void rvbl_test_putc(int, void *);

void rvbl_test_log(const char *format, ...)
{
    va_list arguments;

    va_start(arguments, format);
    npf_vpprintf(rvbl_test_putc, NULL, format, arguments);
    rvbl_test_putc('\n', NULL);
    rvbl_test_putc('\r', NULL);
    va_end(arguments);
}

struct uart_parameters
{
    const rvbl_uart *uart;
    const void *instance;
};

static void rvbl_test_putc_uart(int c, void *p)
{
    const struct uart_parameters *parameters = p;

    while (parameters->uart->tx_ready(parameters->instance, NULL) == rvbl_false) {
    }

    parameters->uart->putc(parameters->instance, c, NULL);
}

void rvbl_test_log_uart(const rvbl_uart *uart, const void *instance, const char *format, ...)
{
    struct uart_parameters parameters = {uart, instance};
    va_list arguments;

    va_start(arguments, format);
    npf_vpprintf(rvbl_test_putc_uart, &parameters, format, arguments);
    rvbl_test_putc_uart('\n', &parameters);
    rvbl_test_putc_uart('\r', &parameters);
    va_end(arguments);
}

void rvbl_hart_hang_if_not(rvbl_uint32_t hart)
{
    if (RVBL_REGISTER_READ(riscv_hart, privileged, &rvbl_riscv_hart_instance_0, mhartid) != hart) {
        rvbl_hang();
    }
}
