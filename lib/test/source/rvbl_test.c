#include "rvbl/test/rvbl_test.h"

#define NANOPRINTF_IMPLEMENTATION
#define NANOPRINTF_USE_FLOAT_FORMAT_SPECIFIERS 0
#define NANOPRINTF_USE_FIELD_WIDTH_FORMAT_SPECIFIERS 1
#define NANOPRINTF_USE_PRECISION_FORMAT_SPECIFIERS 0
#define NANOPRINTF_USE_LARGE_FORMAT_SPECIFIERS 1
#define NANOPRINTF_USE_BINARY_FORMAT_SPECIFIERS 0
#define NANOPRINTF_USE_WRITEBACK_FORMAT_SPECIFIERS 0
#define NANOPRINTF_USE_SMALL_FORMAT_SPECIFIERS 0
#define NANOPRINTF_VISIBILITY_STATIC 1
#include <nanoprintf.h>

#include "rvbl/c/string.h"
#include "rvbl/machine/rvbl_machine.h"

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

void rvbl_hart_hang_if_not(rvbl_uint32_t hart)
{
    if (rvbl_mhartid_read() != hart) {
        rvbl_hang();
    }
}
