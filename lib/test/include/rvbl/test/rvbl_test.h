#ifndef RVBL_TEST_H
#define RVBL_TEST_H

#include "rvbl/core/rvbl_types.h"
#include "rvbl/semihost/rvbl_semihost.h"

typedef enum TestResult
{
    TestResult_Pass = rvbl_semihost_exit_code_success,
    TestResult_Fail = rvbl_semihost_exit_code_error,
    TestResult_Skip = rvbl_semihost_exit_code_skip,
    TestResult_User = rvbl_semihost_exit_code_user
} TestResult;

void rvbl_test_shutdown(TestResult);

#define PASS() rvbl_test_shutdown(TestResult_Pass)
#define FAIL() rvbl_test_shutdown(TestResult_Fail)
#define SKIP() rvbl_test_shutdown(TestResult_Skip)

#define ASSERT(condition)                                                                          \
    if (!(condition)) {                                                                            \
        rvbl_test_log("%s:%d:0: Failed assertion: " #condition, __FILE__, __LINE__);               \
        rvbl_test_shutdown(TestResult_Fail);                                                       \
    } else {                                                                                       \
    }

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

void rvbl_test_cause_store_fault(void);

void rvbl_test_initialize(void);
void rvbl_test_log(const char *format, ...);

void rvbl_hang(void);
void rvbl_hart_hang_if_not(rvbl_uint32_t);

#endif
