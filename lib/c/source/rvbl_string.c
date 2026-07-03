/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/c/string.h"

size_t strlen(const char *str)
{
    size_t i = 0;

    while (str[i] != '\0') {
        ++i;
    }

    return i;
}

void *memset(void *const dest, const int ch, const unsigned int count)
{
    size_t i;

    for (i = 0; i < count; ++i) {
        ((volatile unsigned char *)dest)[i] = (unsigned char)ch;
    }

    return dest;
}

void *memcpy(void *const dest, const void *const src, size_t count)
{
    const char *a = (const char *)src;
    char *b = (char *)dest;

    for (; count > 0; --count, ++a, ++b) {
        *b = *a;
    }

    return dest;
}

int strcmp(const char *lhs, const char *rhs)
{
    while (*lhs && (*lhs == *rhs)) {
        lhs++;
        rhs++;
    }

    return *(const unsigned char *)lhs - *(const unsigned char *)rhs;
}

char *strcpy(char *dest, const char *src) { return memcpy(dest, src, strlen(src) + 1); }
