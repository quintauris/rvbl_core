/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#ifndef RVBL_STRING_H
#define RVBL_STRING_H

/// == <string.h>

#include <stddef.h>

/// === Function `strlen`
/// Returns the length of the given null-terminated byte string.
///
/// ==== Parameters
/// `const char*`:: Pointer to the null-terminated byte string to be examined.
///
/// ==== Return value
/// `size_t`:: The length of the null-terminated byte string.
size_t strlen(const char *);

/// === Function `memset`
///  Copies the value `unsigned char` into each of the first `size_t` characters
/// of the object pointed to by `void*`.
///
/// ==== Parameters
/// `void*`:: Pointer to the object to fill.
/// `int`:: Fill byte.
/// `size_t`:: Number of bytes to fill.
///
/// ==== Return value
/// `void*`:: A copy of `void*` parameter.
void *memset(void *, int, size_t);

/// === Function `memcpy`
/// Copies `size_t` characters from the object pointed to by `const void*` to
/// the object pointed to by `void*`.
///
/// ==== Parameters
/// `void*`:: Pointer to the object to copy to.
/// `const void*`:: Pointer to the object to copy from.
/// `size_t`:: Number of bytes to copy.
///
/// ==== Return value
/// `void*`:: A copy of `void*` parameter.
void *memcpy(void *, const void *, size_t);

/// === Function `strcmp`
/// Compares two null-terminated byte strings lexicographically.
///
/// ==== Parameters
/// `const char*`:: Pointer to the first null-terminated string to compare.
/// `const char*`:: Pointer to the second null-terminated string to compare.
///
/// ==== Return value
/// `int`:: Negative value if lhs appears before rhs in lexicographical order.
/// Zero if lhs and rhs compare equal. Positive value if lhs appears after rhs
/// in lexicographical order.
int strcmp(const char *lhs, const char *rhs);

/// === Function `strcpy`
/// Copies the null-terminated byte string pointed by `const char*`,
/// including the null terminator, to the character array whose first element
/// is pointed to by `char*`.
///
/// ==== Parameters
/// `char*`:: Pointer to the character array to write to.
/// `const char*`:: Pointer to the null-terminated byte string to copy from.
///
/// ==== Return value
/// `char*`:: A copy of `char*` parameter.
char *strcpy(char *, const char *);

#endif
