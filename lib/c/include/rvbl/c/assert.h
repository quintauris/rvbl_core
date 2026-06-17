/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = C Standard Library
/// Quintauris GmbH
///
/// See https://en.cppreference.com/w/c/header.html.

#ifndef RVBL_ASSERT_H
#define RVBL_ASSERT_H

/// == <assert.h>

/// === Macro `assert`
/// Breaks if the user-specified condition is not `true`.
#define assert(x)                                                                                  \
    if (!(x)) {                                                                                    \
        __asm__ volatile("ebreak");                                                                \
    } else {                                                                                       \
    }

#endif
