/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#ifndef RVBL_STDINT_H
#define RVBL_STDINT_H

/// == <stdint.h>

#include "rvbl/type/rvbl_types.h"

/// === Type `int8_t`
/// Signed 8-bit integer.
typedef rvbl_int8_t int8_t;

/// === Type `int_fast8_t`
/// Signed fast 8-bit integer.
typedef rvbl_int8_t int_fast8_t;

/// === Type `int16_t`
/// Signed 16-bit integer.
typedef rvbl_int16_t int16_t;

/// === Type `int32_t`
/// Signed 32-bit integer.
typedef rvbl_int32_t int32_t;

/// === Type `int64_t`
/// Signed 64-bit integer.
typedef rvbl_int64_t int64_t;

/// === Type `intptr_t`
/// Signed integer type capable of holding a pointer to void.
typedef rvbl_word_t intptr_t;

/// === Type `intmax_t`
/// Maximum-width signed integer type.
typedef rvbl_int64_t intmax_t;

/// === Type `uint8_t`
/// Unsigned 8-bit integer.
typedef rvbl_uint8_t uint8_t;

/// === Type `uint_fast8_t`
/// Unsigned fast 8-bit integer.
typedef rvbl_uint8_t uint_fast8_t;

/// === Type `uint16_t`
/// Unsigned 16-bit integer.
typedef rvbl_uint16_t uint16_t;

/// === Type `uint32_t`
/// Unsigned 32-bit integer.
typedef rvbl_uint32_t uint32_t;

/// === Type `uint64_t`
/// Unsigned 64-bit integer.
typedef rvbl_uint64_t uint64_t;

/// === Type `uintmax_t`
/// Maximum-width unsigned integer type.
typedef rvbl_uint64_t uintmax_t;

/// === Type `uintptr_t`
/// Unsigned integer type capable of holding a pointer to void.
typedef rvbl_uword_t uintptr_t;

/// === Type `ptrdiff_t`
/// Signed integer type of the result of subtracting two pointers.
typedef rvbl_word_t ptrdiff_t;

#endif
