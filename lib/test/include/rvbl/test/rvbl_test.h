/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = Test Library
/// Quintauris GmbH
///
/// Provides convenient types and functions for testing.

#ifndef RVBL_TEST_H
#define RVBL_TEST_H

/// == <rvbl_test.h>

#include "rvbl/alloc/rvbl_alloc.h"
#include "rvbl/semihost/rvbl_semihost.h"
#include "rvbl/type/rvbl_types.h"
#include "rvbl/uart/rvbl_uart.h"

/// === Enumeration `TestResult`
typedef enum TestResult
{
    /// `TestResult_Pass` = `rvbl_semihost_exit_code_success`
    TestResult_Pass = rvbl_semihost_exit_code_success,
    /// `TestResult_Fail` = `rvbl_semihost_exit_code_error`
    TestResult_Fail = rvbl_semihost_exit_code_error,
    /// `TestResult_Skip` = `rvbl_semihost_exit_code_skip`
    TestResult_Skip = rvbl_semihost_exit_code_skip,
    /// `TestResult_User` = `rvbl_semihost_exit_code_user`
    TestResult_User = rvbl_semihost_exit_code_user
} TestResult;

/// === Function `rvbl_test_shutdown`
/// Communicates the runtime environment that test execution has finished.
///
/// ==== Parameters
/// `TestResult`:: Test execution result.
void rvbl_test_shutdown(TestResult);

/// === Macro `PASS()`
/// Finishes execution with "pass" result.
#define PASS() rvbl_test_shutdown(TestResult_Pass)

/// === Macro `FAIL()`
/// Finishes execution with "fail" result.
#define FAIL() rvbl_test_shutdown(TestResult_Fail)

/// === Macro `SKIP()`
/// Finishes execution with "skip" result.
#define SKIP() rvbl_test_shutdown(TestResult_Skip)

/// === Macro `ASSERT()`
/// Checks condition and continues execution if successful, logs error
/// information and fails otherwise.
#define ASSERT(condition)                                                                          \
    if (!(condition)) {                                                                            \
        rvbl_test_log("%s:%d:0: Failed assertion: " #condition, __FILE__, __LINE__);               \
        rvbl_test_shutdown(TestResult_Fail);                                                       \
    } else {                                                                                       \
    }

/// === Macro `ASSERT_EQ()`
/// Checks equality and continues execution if successful, logs error
/// information and fails otherwise.
#define ASSERT_EQ(found, expected)                                                                 \
    if ((expected) != (found)) {                                                                   \
        rvbl_test_log(                                                                             \
            "%s:%d:0: Failed assertion: expected %08X but found %08X",                             \
            __FILE__,                                                                              \
            __LINE__,                                                                              \
            (expected),                                                                            \
            (found)                                                                                \
        );                                                                                         \
        rvbl_test_shutdown(TestResult_Fail);                                                       \
    } else {                                                                                       \
    }

/// === Function `rvbl_test_cause_store_fault`
/// Deliberatedly produces a RISC-V store fault/exception.
void rvbl_test_cause_store_fault(void);

/// === Function `rvbl_test_initialize`
/// Initializes test support backends (e.g. RISC-V Semihosting).
void rvbl_test_initialize(void);

/// === Type `rvbl_test_interrupt_control`
/// Trap capabilities block.
typedef struct rvbl_test_interrupt_control
{
    /// `can_trigger_interrupt`:: Whether this machine supports interrupt
    /// triggering for testing purposes.
    rvbl_bool_t can_trigger_interrupt;

    /// `interrupt_code`:: Code of interrupt triggered by `trigger_interrupt`.
    rvbl_uint32_t interrupt_code;

    /// `prepare_interrupt`:: Pointer to function that prepares triggering of
    /// `interrupt_code`.
    void (*prepare_interrupt)(void);

    /// `trigger_interrupt`:: Pointer to function that triggers `interrupt_code`,
    /// possibly asynchronously.
    void (*trigger_interrupt)(void);

    /// `cleanup_interrupt`:: Pointer to function that cleans-up after
    /// `interrupt_code` has been triggered e.g. clear interrupt-pending
    /// flag(s).
    void (*cleanup_interrupt)(void);
} rvbl_test_interrupt_control;

/// === Function `rvbl_test_log`
/// Formats and writes log message.
///
/// ==== Parameters
/// `const char*`:: Format string (printf-style).
/// `...`:: Format arguments.
void rvbl_test_log(const char *format, ...);

/// === Function `rvbl_test_log_uart`
/// Formats and writes log message to the designated UART driver and instance.
///
/// This function does not require `rvbl_test_initialize()` to have been
/// previously called.
///
/// ==== Parameters
/// `const rvbl_uart*`:: UART driver.
/// `const void*`:: UART instance (must be compatible with driver).
/// `const char*`:: Format string (printf-style).
/// `...`:: Format arguments.
void rvbl_test_log_uart(const rvbl_uart *uart, const void *instance, const char *format, ...);

/// === Function `rvbl_hang`
/// Hangs current HART in an infinite loop.
void rvbl_hang(void);

/// === Function `rvbl_hang_if_not`
/// Continues execution if HART ID matches, calls `rvbl_hang()` otherwise.
///
/// ==== Parameters
/// `rvbl_uint32_t`:: ID of HART that is allowed to continue.
void rvbl_hart_hang_if_not(rvbl_uint32_t);

/// === Global `rvbl_test_allocator`
/// Provides a simple 1K allocator for tests.
extern rvbl_alloc_allocator rvbl_test_allocator;

/// === Global `rvbl_test_machine_interrupt_control`
/// Machine-mode interrupt triggering capabilities, for testing.
extern const rvbl_test_interrupt_control rvbl_test_machine_interrupt_control;

/// === Global `rvbl_test_machine_interrupt_control`
/// Supervisor-mode interrupt triggering capabilities, for testing.
extern const rvbl_test_interrupt_control rvbl_test_supervisor_interrupt_control;

#endif
