/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/alloc/rvbl_alloc.h"
#include "rvbl/type/rvbl_types.h"

void *rvbl_alloc_align(void *pointer, rvbl_uword_t alignment)
{
    const rvbl_uword_t value = (rvbl_uword_t)pointer;
    const rvbl_uword_t aligned = (value - 1u + alignment) & -alignment;

    return (void *)aligned;
}

rvbl_result_t rvbl_alloc_initialize(rvbl_alloc_allocator *const allocator)
{
    rvbl_uword_t area = 0;
    rvbl_result_t result = rvbl_result_success;

    for (; (area < allocator->area_count) && (result == rvbl_result_success); ++area) {
        result = allocator->implementation->initialize(&allocator->areas[area]);
    }

    return result;
}

void *rvbl_alloc(rvbl_alloc_allocator *const allocator, const rvbl_uword_t size)
{
    return rvbl_alloc_aligned(allocator, size, rvbl_alloc_default_alignment);
}

void *rvbl_alloc_aligned(
    rvbl_alloc_allocator *const allocator, const rvbl_uword_t size, const rvbl_uword_t alignment
)
{
    rvbl_uword_t area = 0;
    void *result = NULL;

    for (; (area < allocator->area_count) && (result == NULL); ++area) {
        result = allocator->implementation->allocate(&allocator->areas[area], size, alignment);
    }

    return result;
}

void rvbl_alloc_free(rvbl_alloc_allocator *const allocator, void *const pointer)
{
    rvbl_alloc_free_sized(allocator, pointer, 0);
}

void rvbl_alloc_free_sized(rvbl_alloc_allocator *allocator, void *pointer, rvbl_uword_t size)
{
    rvbl_uword_t area = 0;

    for (; (area < allocator->area_count); ++area) {
        const rvbl_alloc_area *i = &allocator->areas[area];

        if ((pointer >= (void *)i->base) && (pointer < (void *)(i->base + i->size))) {
            allocator->implementation->free(i, pointer, size);
        }
    }
}
