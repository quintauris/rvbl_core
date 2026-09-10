/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = Hardware Access Library
/// Quintauris GmbH
///
/// Implements Hardware Access macros.

#ifndef RVBL_HARDWARE_H
#define RVBL_HARDWARE_H

/// == <rvbl_hardware.h>

#include "rvbl/type/rvbl_types.h"

#define RVBL_INDEXED_REGISTER_READ(p, a, i, r, x) rvbl_##p##_##a##_##r##_read(i, x)
#define RVBL_INDEXED_REGISTER_WRITE(p, a, i, r, x, v) rvbl_##p##_##a##_##r##_write(i, x, v)
#define RVBL_INDEXED_REGISTER_CLEAR(p, a, i, r, x)                                                 \
    {                                                                                              \
        register volatile rvbl_##p##_##a##_##r##_t t = {.w = 0};                                   \
        RVBL_INDEXED_REGISTER_WRITE(p, a, i, r, x, t);                                             \
    }
#define RVBL_INDEXED_REGISTER_SET(p, a, i, r, x)                                                   \
    {                                                                                              \
        register volatile rvbl_##p##_##a##_##r##_t t = {.w = -1};                                  \
        RVBL_INDEXED_REGISTER_WRITE(p, a, i, r, x, t);                                             \
    }
#define RVBL_INDEXED_REGISTER_FIELD_READ(p, a, i, r, x, d)                                         \
    RVBL_INDEXED_REGISTER_READ(p, a, i, r, x).f.d
#define RVBL_INDEXED_REGISTER_FIELD_WRITE(p, a, i, r, x, d, v)                                     \
    {                                                                                              \
        register volatile rvbl_##p##_##a##_##r##_t t = RVBL_INDEXED_REGISTER_READ(p, a, i, r, x);  \
        t.f.d = (v);                                                                               \
        RVBL_INDEXED_REGISTER_WRITE(p, a, i, r, x, t);                                             \
    }
#define RVBL_INDEXED_REGISTER_FIELD_CLEAR(p, a, i, r, x, d)                                        \
    RVBL_INDEXED_REGISTER_FIELD_WRITE(p, a, i, r, x, d, 0)
#define RVBL_INDEXED_REGISTER_FIELD_SET(p, a, i, r, x, d)                                          \
    RVBL_INDEXED_REGISTER_FIELD_WRITE(p, a, i, r, x, d, -1)

#define RVBL_REGISTER_READ(p, a, i, r) RVBL_INDEXED_REGISTER_READ(p, a, i, r, 0)
#define RVBL_REGISTER_WRITE(p, a, i, r, v) RVBL_INDEXED_REGISTER_WRITE(p, a, i, r, 0, v)
#define RVBL_REGISTER_CLEAR(p, a, i, r) RVBL_INDEXED_REGISTER_CLEAR(p, a, i, r, 0)
#define RVBL_REGISTER_SET(p, a, i, r) RVBL_INDEXED_REGISTER_SET(p, a, i, r, 0)
#define RVBL_REGISTER_FIELD_READ(p, a, i, r, d) RVBL_INDEXED_REGISTER_FIELD_READ(p, a, i, r, 0, d)
#define RVBL_REGISTER_FIELD_WRITE(p, a, i, r, d, v)                                                \
    RVBL_INDEXED_REGISTER_FIELD_WRITE(p, a, i, r, 0, d, v)
#define RVBL_REGISTER_FIELD_CLEAR(p, a, i, r, d) RVBL_INDEXED_REGISTER_FIELD_CLEAR(p, a, i, r, 0, d)
#define RVBL_REGISTER_FIELD_SET(p, a, i, r, d) RVBL_INDEXED_REGISTER_FIELD_SET(p, a, i, r, 0, d)

#endif
