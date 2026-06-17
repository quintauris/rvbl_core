/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = Allocation Library
/// Quintauris GmbH

#ifndef RVBL_ALLOC_H
#define RVBL_ALLOC_H

/// == <rvbl_alloc.h>

#include "rvbl/compiler/rvbl_compiler.h"
#include "rvbl/type/rvbl_types.h"

/// === Type `rvbl_alloc_area`
///
/// Memory area for allocators to operate on.
typedef struct rvbl_alloc_area
{
    /// `rvbl_uint8_t *base`:: Base address of the memory area.
    rvbl_uint8_t *base;
    /// `rvbl_uword_t size`:: Size, in bytes, of the memory area.
    rvbl_uword_t size;
} rvbl_alloc_area;

#define RVBL_ALLOC_AREA_FROM_MEMORY_REGION_HELPER(a, b) {.base = (rvbl_uint8_t *)a, .size = b}

/// === Macro `RVBL_ALLOC_AREA_FROM_MEMORY_REGION`
///
/// Inlines `rvbl_alloc_area` definition from a model-defined memory region.
#define RVBL_ALLOC_AREA_FROM_MEMORY_REGION(r)                                                      \
    RVBL_ALLOC_AREA_FROM_MEMORY_REGION_HELPER(                                                     \
        RVBL_MEMORY_REGION_##r##_BASE, RVBL_MEMORY_REGION_##r##_SIZE                               \
    )

/// === Type `rvbl_alloc_initialize_implementation`
///
/// Function type for allocator initialization.
///
/// ==== Parameters
/// `const rvbl_alloc_area*`:: Area to initialize allocator in.
///
/// ==== Return Value
/// `rvbl_result_t`:: `rvbl_result_bounds` if not enough memory is available
/// for the allocator to be initialized, `rvbl_result_success` otherwise.
typedef rvbl_result_t (*rvbl_alloc_initialize_implementation)(const rvbl_alloc_area *);

/// === Type `rvbl_alloc_allocate_implementation`
///
/// Function type for allocator allocation.
///
/// ==== Parameters
/// `const rvbl_alloc_area*`:: Area to allocate memory in.
/// `rvbl_uword_t`:: Number of bytes to allocate.
/// `rvbl_uword_t`:: Alignment of the allocation, in bytes.
///
/// ==== Return Value
/// `void*`:: `NULL` if allocation failed, address of allocation otherwise.
typedef void *(*rvbl_alloc_allocate_implementation)(
    const rvbl_alloc_area *, rvbl_uword_t, rvbl_uword_t
);

/// === Type `rvbl_alloc_free_implementation`
///
/// Function type for allocator release.
///
/// ==== Parameters
/// `const rvbl_alloc_area*`:: Area to free memory from.
/// `void*`:: Address of allocation previously obtained with
/// `rvbl_alloc_initialize_implementation` on the same memory area and allocator
/// implementation.
/// `rvbl_uword_t`:: Hint for size of the object to release, in bytes.
///
/// ==== Return Value
/// `void*`:: `NULL` if allocation failed, address of allocation otherwise.
typedef void (*rvbl_alloc_free_implementation)(const rvbl_alloc_area *, void *, rvbl_uword_t);

/// === Type `rvbl_alloc_implementation`
///
/// Function pointers for allocator implementation.
typedef struct rvbl_alloc_implementation
{
    //// `rvbl_alloc_initialize_implementation initialize`:: Initialization implementation.
    rvbl_alloc_initialize_implementation initialize;
    //// `rvbl_alloc_allocate_implementation allocate`:: Allocation implementation.
    rvbl_alloc_allocate_implementation allocate;
    //// `rvbl_alloc_free_implementation free`:: Free implementation.
    rvbl_alloc_free_implementation free;
} rvbl_alloc_implementation;

/// === Type `rvbl_alloc_allocator`
///
/// Complete allocator definition, inlcuding implementation and memory areas.
typedef struct rvbl_alloc_allocator
{
    //// `const rvbl_alloc_implementation *implementation`:: Initialization implementation.
    const rvbl_alloc_implementation *implementation;
    /// `rvbl_uword_t area_count`:: Number of memory areas.
    rvbl_uword_t area_count;
    /// `rvbl_alloc_area *areas`:: Pointer to array of memory areas.
    rvbl_alloc_area *areas;
} rvbl_alloc_allocator;

/// === Macro `rvbl_alloc_default_alignment`
///
/// Default alignment (machine register size, in bytes).
#define rvbl_alloc_default_alignment (XLEN / 8)

/// === Function `rvbl_alloc_align`
///
/// Aligns an address to a given boundary.
///
/// ==== Parameters
/// `void*`:: Memory address to align.
/// `rvbl_uword_t`:: Desired alignment, in bytes.
///
/// ==== Return Value
/// `void*`:: Received memory address, possibly increased to the nearest aligned
/// value.
void *rvbl_alloc_align(void *, rvbl_uword_t);

/// === Function `rvbl_alloc_initialize`
///
/// Performs allocator initialization.
///
/// ==== Parameters
/// `rvbl_alloc_allocator*`:: Allocator to initialize.
///
/// ==== Return Value
/// `rvbl_result_t`:: `rvbl_result_success` on successful initialization,
/// `rvbl_result_*` otherwise.
rvbl_result_t rvbl_alloc_initialize(rvbl_alloc_allocator *);

/// === Function `rvbl_alloc`
///
/// Allocates memory using default alignment.
///
/// ==== Parameters
/// `rvbl_alloc_allocator*`:: Allocator to initialize.
/// `rvbl_uword_t`:: Number of bytes to allocate.
///
/// ==== Return Value
/// `void*`:: `NULL` if allocation failed, address of allocation otherwise.
void *rvbl_alloc(rvbl_alloc_allocator *, rvbl_uword_t);

/// === Function `rvbl_alloc_aligned`
///
/// Allocates memory with custom alignment.
///
/// ==== Parameters
/// `rvbl_alloc_allocator*`:: Allocator to use.
/// `rvbl_uword_t`:: Number of bytes to allocate.
/// `rvbl_uword_t`:: Alignment of the allocation, in bytes.
///
/// ==== Return Value
/// `void*`:: `NULL` if allocation failed, address of allocation otherwise.
void *rvbl_alloc_aligned(rvbl_alloc_allocator *, rvbl_uword_t, rvbl_uword_t);

/// === Function `rvbl_alloc_free`
///
/// Allocates memory with custom alignment.
///
/// ==== Parameters
/// `rvbl_alloc_allocator*`:: Allocator to use.
/// `void*`:: Address of allocation previously obtained with
/// `rvbl_alloc` or `rvbl_alloc_aligned` on the same allocator implementation.
/// `rvbl_uword_t`:: Hint for size of the object to release, in bytes.
void rvbl_alloc_free(rvbl_alloc_allocator *, void *);

/// === Function `rvbl_alloc_free_sized`
///
/// Allocates memory with custom alignment, providing a hint of the size of
/// the object being deallocated.
///
/// ==== Parameters
/// `rvbl_alloc_allocator*`:: Allocator to use.
/// `void*`:: Address of allocation previously obtained with
/// `rvbl_alloc` or `rvbl_alloc_aligned` on the allocator implementation.
/// `rvbl_uword_t`:: Hint for size of the object to release, in bytes.
void rvbl_alloc_free_sized(rvbl_alloc_allocator *, void *, rvbl_uword_t);

#endif
