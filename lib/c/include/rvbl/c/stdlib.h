/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#ifndef RVBL_STDLIB_H
#define RVBL_STDLIB_H

#include <stddef.h>

/// == <stdlib.h>

/// === Function `abs`
/// Computes the absolute value of an integer number.
///
/// ==== Parameters
/// `int`:: Integer value.
///
/// ==== Return value
/// `int`:: The absolute value.
int abs(int n);

/// === Function `labs`
/// Computes the absolute value of an integer number.
///
/// ==== Parameters
/// `long`:: Integer value.
///
/// ==== Return value
/// `long`:: The absolute value.
long labs(long n);

/// === Function `malloc`
/// Allocates size bytes of uninitialized storage.
///
/// ==== Parameters
/// `size_t`:: Number of bytes to allocate.
///
/// ==== Return value
/// `long`:: On success, returns the pointer to the beginning of newly allocated
///  memory. To avoid a memory leak, the returned pointer must be deallocated
///  with free() or realloc(). On failure, returns a null pointer.
void *malloc(size_t);

/// === Function `realloc`
/// Reallocates the given area of memory. If ptr is not NULL, it must be
/// previously allocated by malloc, calloc or realloc and not yet freed with a
/// call to free or realloc. Otherwise, the results are undefined..
///
/// ==== Parameters
/// `void*`:: Pointer to the memory area to be reallocated.
/// `size_t`:: New size of the array in bytes.
///
/// ==== Return value
/// `long`:: On success, returns the pointer to the beginning of newly allocated
/// memory. To avoid a memory leak, the returned pointer must be deallocated with free
/// or realloc. The original pointer ptr is invalidated and any access to it is
/// undefined behavior (even if reallocation was in-place). On failure, returns a
/// null pointer. The original pointer ptr remains valid and may need to be
/// deallocated with free or realloc.
void *realloc(void *, size_t);

/// === Function `free`
/// Deallocates the space previously allocated by malloc(), calloc() or realloc().
///
/// ==== Parameters
/// `void*`:: Pointer to the memory to deallocate.
void free(void *);

#endif
