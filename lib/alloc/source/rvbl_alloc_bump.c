/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/alloc/rvbl_alloc_bump.h"
#include "rvbl/alloc/rvbl_alloc.h"
#include "rvbl/type/rvbl_types.h"

typedef struct bump_control
{
    rvbl_uword_t offset;
} bump_control;

static bump_control *bump_control_get(const rvbl_alloc_area *area)
{
    return (bump_control *)(area->base + area->size - sizeof(bump_control));
}

static rvbl_uword_t bump_free_bytes(const rvbl_alloc_area *area, bump_control *control)
{
    return area->size - sizeof(bump_control) - control->offset;
}

static rvbl_result_t bump_initialize(const rvbl_alloc_area *area)
{
    if (area->size >= sizeof(bump_control)) {
        bump_control *control = bump_control_get(area);

        control->offset = 0;

        return rvbl_result_success;
    } else {
        return rvbl_result_error_bounds;
    }
}

static void *bump_allocate(const rvbl_alloc_area *area, rvbl_uword_t size, rvbl_uword_t alignment)
{
    bump_control *control = bump_control_get(area);

    if (size <= bump_free_bytes(area, control)) {
        rvbl_uint8_t *unaligned = area->base + control->offset;
        rvbl_uint8_t *aligned = rvbl_alloc_align(unaligned, alignment);

        control->offset += size + (aligned - unaligned);

        return aligned;
    } else {
        return NULL;
    }
}

static void bump_free(const rvbl_alloc_area *area, void *pointer, rvbl_uword_t size)
{
    (void)area;
    (void)pointer;
    (void)size;
}

const rvbl_alloc_implementation rvbl_alloc_bump_allocator = {
    .initialize = &bump_initialize, .allocate = &bump_allocate, .free = &bump_free
};
