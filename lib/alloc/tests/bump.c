/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/alloc/rvbl_alloc.h"
#include "rvbl/alloc/rvbl_alloc_bump.h"
#include "rvbl/test/rvbl_test.h"

static rvbl_uint8_t heap[128];

int main(void)
{
    rvbl_alloc_area area = {heap, sizeof(heap)};
    rvbl_alloc_allocator allocator = {&rvbl_alloc_bump_allocator, 1, &area};
    void *allocation_default = NULL, *allocation_32 = NULL;

    rvbl_hart_hang_if_not(0);
    rvbl_test_initialize();

    ASSERT_EQ(rvbl_alloc_initialize(&allocator), rvbl_result_success);

    ASSERT((allocation_default = rvbl_alloc(&allocator, 4)) != NULL);
    ASSERT(allocation_default >= (void *)area.base);
    ASSERT(allocation_default < (void *)(area.base + area.size));
    ASSERT(
        rvbl_alloc_align(allocation_default, rvbl_alloc_default_alignment) == allocation_default
    );

    ASSERT((allocation_32 = rvbl_alloc_aligned(&allocator, 4, 32)) != NULL);
    ASSERT(allocation_32 >= (void *)area.base);
    ASSERT(allocation_32 < (void *)(area.base + area.size));
    ASSERT(rvbl_alloc_align(allocation_32, 32) == allocation_32);

    ASSERT(allocation_default != allocation_32)

    PASS();

    return 0;
}
