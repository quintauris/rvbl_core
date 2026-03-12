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

/// === Function `rvbl_test_initialize`
/// Formats and writes log message.
///
/// ==== Parameters
/// `const char*`:: Format string (printf-style).
/// `...`:: Format arguments.
void rvbl_test_log(const char *format, ...);

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

#endif
