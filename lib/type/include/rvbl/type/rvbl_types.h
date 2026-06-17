/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = Types Library
/// Quintauris GmbH
///
/// Implements Core types and ABI-related functions.

#ifndef RVBL_TYPES_H
#define RVBL_TYPES_H

/// == <rvbl_types.h>

#include "rvbl/compiler/rvbl_compiler.h"

/// === Enumeration `rvbl_bool_t`
typedef enum rvbl_bool_t
{
    /// * `rvbl_true` = `1`
    rvbl_true = 1,
    /// * `rvbl_false` = `0`
    rvbl_false = 0
} rvbl_bool_t;

/// === Type `rvbl_int8_t`
/// Signed 8-bit integer.
typedef signed char rvbl_int8_t;

/// === Type `rvbl_int16_t`
/// Signed 16-bit integer.
typedef signed short rvbl_int16_t;

/// === Type `rvbl_int32_t`
/// Signed 32-bit integer.
typedef signed int rvbl_int32_t;

/// === Type `rvbl_word_t`
/// Signed integer of the native word size.
typedef signed long rvbl_word_t;

/// === Type `rvbl_int64_t`
/// Signed 64-bit integer.
typedef signed long long rvbl_int64_t;

/// === Type `rvbl_uint8_t`
/// Unsigned 8-bit integer.
typedef unsigned char rvbl_uint8_t;

/// === Type `rvbl_uint16_t`
/// Unsigned 16-bit integer.
typedef unsigned short rvbl_uint16_t;

/// === Type `rvbl_uint32_t`
/// Unsigned 32-bit integer.
typedef unsigned int rvbl_uint32_t;

/// === Type `rvbl_uword_t`
/// Unsigned integer of the native word size.
typedef unsigned long rvbl_uword_t;

/// === Type `rvbl_uint64_t`
/// Unsigned 64-bit integer.
typedef unsigned long long rvbl_uint64_t;

/// === Type `rvbl_pointer_t`
/// Generic pointer type.
typedef void *rvbl_pointer_t;

/// === Enumeration `rvbl_result_t`
/// General result type supporting success and general errors.
typedef enum rvbl_result_t
{
    /// * `rvbl_result_success` = `0`
    rvbl_result_success = 0,
    /// * `rvbl_result_error_unspecified` = `1`
    rvbl_result_error_unspecified = 1,
    /// * `rvbl_result_error_bounds` = `2`
    rvbl_result_error_bounds = 2,
    /// * `rvbl_result_error_unsupported` = `3`
    rvbl_result_error_unsupported = 3,
    /// * `rvbl_result_error_busy` = `4`
    rvbl_result_error_busy = 4,
    /// * `rvbl_result_error_state` = `5`
    rvbl_result_error_state = 5,
    /// * `rvbl_result_error_out_of_memory` = `6`
    rvbl_result_error_out_of_memory = 6
} rvbl_result_t;

#endif
