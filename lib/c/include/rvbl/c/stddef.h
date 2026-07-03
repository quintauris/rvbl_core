/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#ifndef RVBL_STDDEF_H
#define RVBL_STDDEF_H

/// == <stddef.h>

#include "rvbl/type/rvbl_types.h"

/// === Type `size_t`
/// Unsigned integer type of the result of sizeof, offsetof.
typedef rvbl_uint32_t size_t;

/// === Macro `offsetof`
///
/// Expands to an integral constant expression of type std::size_t, the value of
/// which is the offset, in bytes, from the beginning of an object of specified
/// type to its specified subobject, including padding bits if any.
#define offsetof(a, b) __builtin_offsetof(a, b)

#endif
