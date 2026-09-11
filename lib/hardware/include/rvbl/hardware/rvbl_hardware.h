/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

/// = Hardware Access Library
/// Quintauris GmbH
///
/// Implements Hardware Access macros.

#ifndef RVBL_HARDWARE_H
#define RVBL_HARDWARE_H

/// == <rvbl_hardware.h>

#include "rvbl/type/rvbl_types.h"

/// === Macro `RVBL_INDEXED_REGISTER_READ`
/// Returns the value of an indexed register.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `ind`:: Index to access.
///
/// ==== Return value
/// `<register type>`:: Register value.
#define RVBL_INDEXED_REGISTER_READ(per, addr, inst, reg, ind)                                      \
    rvbl_##per##_##addr##_##reg##_read(inst, ind)

/// === Macro `RVBL_INDEXED_REGISTER_WRITE`
/// Sets the value of an indexed register.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `ind`:: Index to access.
/// `val`:: Register value to set.
#define RVBL_INDEXED_REGISTER_WRITE(per, addr, inst, reg, ind, val)                                \
    rvbl_##per##_##addr##_##reg##_write(inst, ind, val)

/// === Macro `RVBL_INDEXED_REGISTER_CLEAR`
/// Zeroes the value of an indexed register.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `ind`:: Index to access.
#define RVBL_INDEXED_REGISTER_CLEAR(per, addr, inst, reg, ind)                                     \
    {                                                                                              \
        register volatile rvbl_##per##_##addr##_##reg##_t t = {.w = 0};                            \
        RVBL_INDEXED_REGISTER_WRITE(per, addr, inst, reg, ind, t);                                 \
    }

/// === Macro `RVBL_INDEXED_REGISTER_SET`
/// Sets the value of an indexed register to all ones.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `ind`:: Index to access.
#define RVBL_INDEXED_REGISTER_SET(per, addr, inst, reg, ind)                                       \
    {                                                                                              \
        register volatile rvbl_##per##_##addr##_##reg##_t t = {.w = -1};                           \
        RVBL_INDEXED_REGISTER_WRITE(per, addr, inst, reg, ind, t);                                 \
    }

/// === Macro `RVBL_INDEXED_REGISTER_FIELD_READ`
/// Returns the value of an indexed register's field.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `ind`:: Index to access.
/// `fie`:: Register field name in machine model.
///
/// ==== Return value
/// `<field type>`:: Register field value.
#define RVBL_INDEXED_REGISTER_FIELD_READ(per, addr, inst, reg, ind, fie)                           \
    RVBL_INDEXED_REGISTER_READ(per, addr, inst, reg, ind).f.fie

/// === Macro `RVBL_INDEXED_REGISTER_FIELD_WRITE`
/// Sets the value of an indexed register's field.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `ind`:: Index to access.
/// `fie`:: Register field name in machine model.
/// `val`:: Register field value to set.
#define RVBL_INDEXED_REGISTER_FIELD_WRITE(per, addr, inst, reg, ind, fie, val)                     \
    {                                                                                              \
        register volatile rvbl_##per##_##addr##_##reg##_t t =                                      \
            RVBL_INDEXED_REGISTER_READ(per, addr, inst, reg, ind);                                 \
        t.f.fie = (val);                                                                           \
        RVBL_INDEXED_REGISTER_WRITE(per, addr, inst, reg, ind, t);                                 \
    }

/// === Macro `RVBL_INDEXED_REGISTER_FIELD_CLEAR`
/// Zeroes the value of an indexed register's field.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `ind`:: Index to access.
/// `fie`:: Register field name in machine model.
#define RVBL_INDEXED_REGISTER_FIELD_CLEAR(per, addr, inst, reg, ind, fie)                          \
    RVBL_INDEXED_REGISTER_FIELD_WRITE(per, addr, inst, reg, ind, fie, 0)

/// === Macro `RVBL_INDEXED_REGISTER_FIELD_SET`
/// Sets the value of an indexed register's field to all ones.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `ind`:: Index to access.
/// `fie`:: Register field name in machine model.
#define RVBL_INDEXED_REGISTER_FIELD_SET(per, addr, inst, reg, ind, fie)                            \
    RVBL_INDEXED_REGISTER_FIELD_WRITE(per, addr, inst, reg, ind, fie, -1)

/// === Macro `RVBL_REGISTER_READ`
/// Returns the value of a register.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
///
/// ==== Return value
/// `<register type>`:: Register value.
#define RVBL_REGISTER_READ(per, addr, inst, reg) RVBL_INDEXED_REGISTER_READ(per, addr, inst, reg, 0)

/// === Macro `RVBL_REGISTER_WRITE`
/// Sets the value of a register.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `val`:: Register value to set.
#define RVBL_REGISTER_WRITE(per, addr, inst, reg, val)                                             \
    RVBL_INDEXED_REGISTER_WRITE(per, addr, inst, reg, 0, val)

/// === Macro `RVBL_REGISTER_CLEAR`
/// Zeroes the value of a register.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
#define RVBL_REGISTER_CLEAR(per, addr, inst, reg)                                                  \
    RVBL_INDEXED_REGISTER_CLEAR(per, addr, inst, reg, 0)

/// === Macro `RVBL_REGISTER_SET`
/// Sets the value of a register to all ones.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
#define RVBL_REGISTER_SET(per, addr, inst, reg) RVBL_INDEXED_REGISTER_SET(per, addr, inst, reg, 0)

/// === Macro `RVBL_REGISTER_FIELD_READ`
/// Returns the value of a register's field.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `fie`:: Register field name in machine model.
///
/// ==== Return value
/// `<field type>`:: Register field value.
#define RVBL_REGISTER_FIELD_READ(per, addr, inst, reg, fie)                                        \
    RVBL_INDEXED_REGISTER_FIELD_READ(per, addr, inst, reg, 0, fie)

/// === Macro `RVBL_REGISTER_FIELD_WRITE`
/// Sets the value of a register's field.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `fie`:: Register field name in machine model.
/// `val`:: Register field value to set.
#define RVBL_REGISTER_FIELD_WRITE(per, addr, inst, reg, fie, val)                                  \
    RVBL_INDEXED_REGISTER_FIELD_WRITE(per, addr, inst, reg, 0, fie, val)

/// === Macro `RVBL_REGISTER_FIELD_CLEAR`
/// Zeroes the value of a register's field.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `fie`:: Register field name in machine model.
#define RVBL_REGISTER_FIELD_CLEAR(per, addr, inst, reg, fie)                                       \
    RVBL_INDEXED_REGISTER_FIELD_CLEAR(per, addr, inst, reg, 0, fie)

/// === Macro `RVBL_REGISTER_FIELD_SET`
/// Sets the value of a register's field to all ones.
///
/// ==== Parameters
/// `per`:: Peripheral name in machine model.
/// `addr`:: Addressable name within peripheral in machine model.
/// `inst`:: Peripheral instance name in machine model.
/// `reg`:: Register name in machine model.
/// `fie`:: Register field name in machine model.
#define RVBL_REGISTER_FIELD_SET(per, addr, inst, reg, fie)                                         \
    RVBL_INDEXED_REGISTER_FIELD_SET(per, addr, inst, reg, 0, fie)

#endif
