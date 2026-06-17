/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/alloc/rvbl_alloc_bump.h"
#include "rvbl/c/stdlib.h"

int abs(int n) { return (n < 0) ? -n : n; }

long labs(long n) { return (n < 0) ? -n : n; }

extern rvbl_alloc_allocator rvbl_c_stdlib_allocator;

void *malloc(size_t count) { return rvbl_alloc(&rvbl_c_stdlib_allocator, count); }

void *realloc(void *pointer, size_t size)
{
    (void)pointer;
    (void)size;
    return NULL;
}

void free(void *pointer) { rvbl_alloc_free(&rvbl_c_stdlib_allocator, pointer); }
