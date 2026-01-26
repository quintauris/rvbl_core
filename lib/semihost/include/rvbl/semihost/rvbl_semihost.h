#ifndef RVBL_SEMIHOST_H
#define RVBL_SEMIHOST_H

#include "rvbl/core/rvbl_types.h"

/// = RISC-V Semihost Library
/// Quintauris GmbH
/// :toc: left
///
/// Implements semihost-related types and functions.

/// == Function `rvbl_semihost_initialize`
///
/// Initializes the semihosting environment.
void rvbl_semihost_initialize(void);

/// == Function `rvbl_semihost_assume`
///
/// Initializes the semihosting environment without any run-time checks.
void rvbl_semihost_assume(void);

/// == Type `rvbl_semihost_exit_code`
///
/// Valid exit codes for semihost EXIT operation.
typedef enum rvbl_semihost_exit_code
{
    rvbl_semihost_exit_code_success = 0,
    rvbl_semihost_exit_code_error = 1,
    rvbl_semihost_exit_code_skip = 2,
    rvbl_semihost_exit_code_user = 3
} rvbl_semihost_exit_code;

/// == Type `rvbl_semihost_file_handle`
///
/// Handle type for semihosting OPEN operation.
typedef rvbl_uword_t rvbl_semihost_file_handle;

/// == Type `rvbl_semihost_file_mode`
///
/// Valid file open modes for semihosting OPEN operation.
typedef enum rvbl_semihost_file_mode
{
    rvbl_semihost_file_mode_read_binary = 1,
    rvbl_semihost_file_mode_write_plus = 6
} rvbl_semihost_file_mode;

/// == Function `rvbl_semihost_output_stream_open`
///
/// Opens a file through the semihosting interface.
///
/// === Parameters
/// `const char*`:: File to open (use `":tt"` for stdin/stdout).
/// `rvbl_semihost_file_mode`:: File open mode.
///
/// === Return value
/// `rvbl_semihost_file_handle`:: Resulting open file handle.
rvbl_semihost_file_handle rvbl_semihost_output_stream_open(const char *, rvbl_semihost_file_mode);

/// == Function `rvbl_semihost_output_stream_write`
///
/// Writes to an open file through the semihosting interface.
///
/// === Parameters
/// `rvbl_semihost_file_handle`:: Target open file handle.
/// `const rvbl_uint8_t*`:: Data to write.
/// `rvbl_uword_t`:: Number of bytes in previous parameter.
void rvbl_semihost_output_stream_write(
    rvbl_semihost_file_handle, const rvbl_uint8_t *, rvbl_uword_t
);

/// == Function `rvbl_semihost_output_stream_write`
///
/// Closes an open file through the semihosting interface.
///
/// === Parameters
/// `rvbl_semihost_file_handle`:: Target open file handle.
void rvbl_semihost_output_stream_close(rvbl_semihost_file_handle);

/// == Function `rvbl_semihost_exit`
///
/// Performs a semihosting exit operation.
///
/// === Parameters
/// `rvbl_semihost_exit_code`:: Exit code to pass to the semihosting.
void rvbl_semihost_exit(rvbl_semihost_exit_code);

#endif
