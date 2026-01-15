#ifndef RVBL_STRING_H
#define RVBL_STRING_H

/// = RISC-V Base Layer C Library
/// Quintauris GmbH
/// :toc: left
///
/// Implements parts of the C standard library required by the Base Layer.

#include <stddef.h>

/// == Function `strlen`
/// Returns the length of the given null-terminated byte string.
///
/// === Parameters
/// `const char*`:: Pointer to the null-terminated byte string to be examined.
///
/// === Return value
/// `size_t`:: The length of the null-terminated byte string.
size_t strlen(const char *);

/// == Function `memset`
///  Copies the value `unsigned char` into each of the first `size_t` characters
/// of the object pointed to by `void*`.
///
/// === Parameters
/// `void*`:: Pointer to the object to fill.
/// `int`:: Fill byte.
/// `size_t`:: Number of bytes to fill.
///
/// === Return value
/// `void*`:: A copy of `void*` parameter.
void *memset(void *, int, size_t);

/// == Function `memcpy`
/// Copies `size_t` characters from the object pointed to by `const void*` to
/// the object pointed to by `void*`.
///
/// === Parameters
/// `void*`:: Pointer to the object to copy to.
/// `const void*`:: Pointer to the object to copy from.
/// `size_t`:: Number of bytes to copy.
///
/// === Return value
/// `void*`:: A copy of `void*` parameter.
void *memcpy(void *, const void *, size_t);

/// == Function `strcmp`
/// Compares two null-terminated byte strings lexicographically.
///
/// === Parameters
/// `const char*`:: Pointer to the first null-terminated string to compare.
/// `const char*`:: Pointer to the second null-terminated string to compare.
///
/// === Return value
/// `int`:: Negative value if lhs appears before rhs in lexicographical order.
/// Zero if lhs and rhs compare equal. Positive value if lhs appears after rhs
/// in lexicographical order.
int strcmp(const char *lhs, const char *rhs);

#endif