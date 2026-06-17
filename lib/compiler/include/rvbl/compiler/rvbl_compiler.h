/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = Compiler Abstraction Library
/// Quintauris GmbH
///
/// Implements Compiler Abstraction macros.

#ifndef _RVBL_COMPILER_H_
#define _RVBL_COMPILER_H_

/// == <rvbl_compiler.h>

#define __xstr(x) #x
#define __str(x) __xstr(x)

/// === Macro `RVBL_INLINE`
///
/// Begins function that will be always inlined.
///
/// === Macro `RVBL_BEGIN_DISABLE_OPTIMIZATION`
///
/// Begins code section where the compiler shall perform no optimization.
///
/// === Macro `RVBL_END_DISABLE_OPTIMIZATION`
///
/// Ends code section where the compiler shall perform no optimization.
#ifdef __clang__
#define RVBL_INLINE() __attribute__((always_inline)) static inline
#define RVBL_BEGIN_DISABLE_OPTIMIZATION() _Pragma("clang optimize off")
#define RVBL_END_DISABLE_OPTIMIZATION() _Pragma("clang optimize on")
#elif __TASKING__
#define RVBL_INLINE() __attribute__((always_inline)) static inline
#define RVBL_BEGIN_DISABLE_OPTIMIZATION() _Pragma("optimize 0")
#define RVBL_END_DISABLE_OPTIMIZATION() _Pragma("endoptimize")
#elif __GNUC__
#define RVBL_INLINE() __attribute__((always_inline)) static inline
#define RVBL_BEGIN_DISABLE_OPTIMIZATION() _Pragma("GCC push_options") _Pragma("GCC optimize 0")
#define RVBL_END_DISABLE_OPTIMIZATION() _Pragma("GCC pop_options")
#elif __IAR_SYSTEMS_ICC__
/* __attribute__((always_inline)) causes compiler crash (IAR bug ID RISCV-4845). */
#define RVBL_INLINE() _Pragma("inline=forced") static inline
#define RVBL_BEGIN_DISABLE_OPTIMIZATION() _Pragma("optimize=balanced none")
#define RVBL_END_DISABLE_OPTIMIZATION() _Pragma("optimize=balanced medium")
#endif

/// === Macro `XLEN`
///
/// Native machine word size (32, 64 or 128-bits).
#define XLEN (sizeof(long) * 8)

/// === Macro `NULL`
///
/// Pointer to `0x00000000`.
#ifndef NULL
#define NULL ((void *)0)
#endif

#endif
