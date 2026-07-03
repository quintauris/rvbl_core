/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = Bump Allocator Library
/// Quintauris GmbH

#ifndef RVBL_ALLOC_BUMP_H
#define RVBL_ALLOC_BUMP_H

/// == <rvbl_alloc_bump.h>

#include "rvbl/alloc/rvbl_alloc.h"

/// === Constant `rvbl_alloc_bump_allocator`
///
/// Implementation pointers for _bump_ heap allocator.
extern const rvbl_alloc_implementation rvbl_alloc_bump_allocator;

#endif
