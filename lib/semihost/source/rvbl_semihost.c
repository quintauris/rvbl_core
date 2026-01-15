#include "rvbl/semihost/rvbl_semihost.h"
#include "rvbl/c/string.h"
#include "rvbl/core/rvbl_types.h"

typedef enum rvbl_semihost_operation
{
    rvbl_semihost_operation_open = 0x01,
    rvbl_semihost_operation_close = 0x02,
    rvbl_semihost_operation_writec = 0x03,
    rvbl_semihost_operation_write = 0x05,
    rvbl_semihost_operation_read = 0x06,
    rvbl_semihost_operation_flen = 0x0C,
    rvbl_semihost_operation_exit = 0x18,
    rvbl_semihost_operation_exit_extended = 0x20
} rvbl_semihost_operation;

extern rvbl_uword_t rvbl_semihost_interface(rvbl_semihost_operation, const rvbl_uword_t *);
static rvbl_bool_t semihost_supported = rvbl_false;
static rvbl_bool_t semihost_exit_extended_supported = rvbl_false;
static rvbl_bool_t semihost_file_access_supported = rvbl_false;

RVBL_BEGIN_DISABLE_OPTIMIZATION()
rvbl_uword_t __attribute__((noinline))
SEGGER_SEMIHOST_DebugHalt(rvbl_semihost_operation r0, const rvbl_uword_t *r1)
{
    (void)r1;
    return r0;
}
RVBL_BEGIN_DISABLE_OPTIMIZATION()

void rvbl_semihost_initialize(void)
{
    const char *stdout_path = ":tt";
    const char *features_path = ":semihosting-features";
    rvbl_uword_t parameters[3];
    rvbl_semihost_file_handle handle = -1;
    size_t size = 0;
    rvbl_uint8_t bytes[8];

    parameters[0] = (rvbl_uword_t)stdout_path;
    parameters[1] = rvbl_semihost_file_mode_write_plus;
    parameters[2] = strlen(stdout_path);
    handle = SEGGER_SEMIHOST_DebugHalt(rvbl_semihost_operation_open, parameters);

    if (handle != rvbl_semihost_operation_open) {
        return;
    }

    parameters[0] = (rvbl_uword_t)features_path;
    parameters[1] = rvbl_semihost_file_mode_read_binary;
    parameters[2] = strlen(features_path);
    handle = rvbl_semihost_interface(rvbl_semihost_operation_open, parameters);

    if (handle == -1) {
        return;
    }

    parameters[0] = handle;
    size = rvbl_semihost_interface(rvbl_semihost_operation_flen, parameters);

    if ((size == 0xFFFFFFFF) || (size < 4)) {
        return;
    }

    parameters[0] = handle;
    parameters[1] = (rvbl_uword_t)bytes;
    parameters[2] = sizeof(bytes);
    size = sizeof(bytes) - rvbl_semihost_interface(rvbl_semihost_operation_read, parameters);

    if (size < 4) {
        return;
    }

    if ((bytes[0] != 0x53) || (bytes[1] != 0x48) || (bytes[2] != 0x46) || (bytes[3] != 0x42)) {
        return;
    }

    semihost_supported = rvbl_true;
    semihost_file_access_supported = rvbl_true;

    if ((size >= 5) && (bytes[4] & 1)) {
        semihost_exit_extended_supported = rvbl_true;
    }
}

void rvbl_semihost_assume(void)
{
    semihost_supported = rvbl_true;
    semihost_exit_extended_supported = rvbl_false;
    semihost_file_access_supported = rvbl_false;
}

void rvbl_semihost_exit(const rvbl_semihost_exit_code code)
{
    const rvbl_uword_t codes[] = {0x20026, code};

    if (semihost_supported) {
        if (semihost_exit_extended_supported) {
            rvbl_semihost_interface(rvbl_semihost_operation_exit_extended, codes);
        } else {
            rvbl_semihost_interface(rvbl_semihost_operation_exit, (const rvbl_uword_t *)code);
        }
    } else {
        SEGGER_SEMIHOST_DebugHalt(rvbl_semihost_operation_exit, (const rvbl_uword_t *)codes);
    }
}

rvbl_semihost_file_handle
rvbl_semihost_output_stream_open(const char *path, const rvbl_semihost_file_mode mode)
{
    rvbl_uword_t parameters[] = {(rvbl_uword_t)path, mode, strlen(path)};

    if (semihost_supported) {
        if (semihost_file_access_supported) {
            return rvbl_semihost_interface(rvbl_semihost_operation_open, parameters);
        } else {
            return 0;
        }
    } else {
        return SEGGER_SEMIHOST_DebugHalt(rvbl_semihost_operation_open, parameters);
    }
}

void rvbl_semihost_output_stream_write(
    const rvbl_semihost_file_handle handle, const rvbl_uint8_t *bytes, const rvbl_uword_t count
)
{
    const rvbl_uword_t parameters[] = {handle, (rvbl_uword_t)bytes, count};

    if (semihost_supported) {
        if (semihost_file_access_supported) {
            rvbl_semihost_interface(rvbl_semihost_operation_write, parameters);
        } else {
            rvbl_uword_t i;

            for (i = 0; i < count; ++i) {
                rvbl_semihost_interface(
                    rvbl_semihost_operation_writec, (const rvbl_uword_t *)(bytes + i)
                );
            }
        }
    } else {
        SEGGER_SEMIHOST_DebugHalt(rvbl_semihost_operation_write, parameters);
    }
}

void rvbl_semihost_output_stream_close(const rvbl_semihost_file_handle handle)
{
    if (semihost_supported) {
        if (semihost_file_access_supported) {
            rvbl_semihost_interface(rvbl_semihost_operation_close, (const rvbl_uword_t *)handle);
        }
    } else {
        SEGGER_SEMIHOST_DebugHalt(rvbl_semihost_operation_close, (const rvbl_uword_t *)handle);
    }
}