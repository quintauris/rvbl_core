/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = Boot Library
/// Quintauris GmbH
///
/// Provides bootstrapping types and functions.

#ifndef RVBL_BOOT_H
#define RVBL_BOOT_H

/// == <rvbl_boot.h>

#include "rvbl/type/rvbl_types.h"

/// === Enumeration `rvbl_boot_action`
/// Defines boot sequence actions.
typedef enum rvbl_boot_action
{
    /// * `rvbl_boot_action_fill` = `0`
    rvbl_boot_action_fill,
    /// * `rvbl_boot_action_copy` = `1`
    rvbl_boot_action_copy,
    /// * `rvbl_boot_action_call` = `2`
    rvbl_boot_action_call,
    /// * `rvbl_boot_action_finish` = `3`
    rvbl_boot_action_finish
} rvbl_boot_action;

/// === Type `rvbl_boot_action_fill_parameters`
/// Defines parameters for "fill" boot operation.
typedef struct rvbl_boot_action_fill_parameters
{
    /// `rvbl_pointer_t begin_address`:: Begin address of fill (word-aligned).
    rvbl_pointer_t begin_adddress;
    /// `rvbl_pointer_t end_address`:: End address of fill (word-aligned).
    rvbl_pointer_t end_adddress;
    /// `rvbl_uword_t value`:: Value to fill with.
    rvbl_uword_t value;
} rvbl_boot_action_fill_paramters;

/// === Type `rvbl_boot_action_copy_parameters`
/// Defines parameters for "copy" boot operation.
typedef struct rvbl_boot_action_copy_parameters
{
    /// `rvbl_pointer_t begin_address`:: Begin address of copy (word-aligned).
    rvbl_pointer_t begin_adddress;
    /// `rvbl_pointer_t end_address`:: End address of copy (word-aligned).
    rvbl_pointer_t end_adddress;
    /// `rvbl_pointer_t target_address`:: Target address of copy (word-aligned).
    rvbl_pointer_t target_address;
} rvbl_boot_action_copy_paramters;

typedef void (*rvbl_boot_action_callout)(rvbl_uword_t, rvbl_uword_t);

/// === Type `rvbl_boot_action_call_parameters`
/// Defines parameters for "call" boot operation.
typedef struct rvbl_boot_action_call_parameters
{
    /// `rvbl_boot_action_callout callout`:: Call target.
    rvbl_boot_action_callout callout;
    /// `rvbl_uword_t parameter1`:: Call first parameter.
    rvbl_uword_t parameter1;
    /// `rvbl_uword_t parameter2`:: Call second parameter.
    rvbl_uword_t parameter2;
} rvbl_boot_action_call_paramters;

/// === Type `rvbl_boot_action_parameters`
/// Defines aggregated parameters for boot operations.
typedef union rvbl_boot_action_parameters
{
    /// `rvbl_boot_action_fill_parameters fill`:: Fill parameters.
    struct rvbl_boot_action_fill_parameters fill;
    /// `rvbl_boot_action_copy_parameters copy`:: Copy parameters.
    struct rvbl_boot_action_copy_parameters copy;
    /// `rvbl_boot_action_call_parameters call`:: Call parameters.
    struct rvbl_boot_action_call_parameters call;
} rvbl_boot_action_parameters;

/// === Type `rvbl_boot_step`
/// Defines a single boot step action and parameters.
typedef struct rvbl_boot_step
{
    /// `rvbl_boot_action action`:: Boot step action.
    rvbl_boot_action action;
    /// `rvbl_boot_action_parameters parameters`:: Boot step action parameters.
    rvbl_boot_action_parameters parameters;
} rvbl_boot_step;

/// === Function `rvbl_boot`
/// Executes boot steps defined by `const rvbl_boot_step boot_steps[]`.
void rvbl_boot(void);

#endif
