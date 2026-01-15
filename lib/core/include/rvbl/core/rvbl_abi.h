#ifndef RVBL_ABI_H
#define RVBL_ABI_H

#include "rvbl/core/rvbl_types.h"

/// == Function `rvbl_thread_pointer_register_read`
/// Returns the current value of the `tp` (`x4`) register.
///
/// === Return value
/// `rvbl_pointer_t`:: Current value of the `tp` (`x4`) register.
rvbl_pointer_t rvbl_thread_pointer_register_read(void);

/// == Function `rvbl_thread_pointer_register_write`
/// Sets the current value of the `tp` (`x4`) register.
///
/// === Parameters
/// `rvbl_pointer_t` `value`:: New value of the `tp` (`x4`) register.
void rvbl_thread_pointer_register_write(rvbl_pointer_t value);

/// == Function `rvbl_stack_pointer_register_read`
/// Returns the current value of the `sp` (`x2`) register.
///
/// === Return value
/// `rvbl_pointer_t`:: Current value of the `sp` (`x2`) register.
rvbl_pointer_t rvbl_stack_pointer_register_read(void);

/// == Function `rvbl_stack_pointer_register_write`
/// Sets the current value of the `sp` (`x2`) register.
///
/// === Parameters
/// `rvbl_pointer_t` `value`:: New value of the `sp` (`x2`) register.
void rvbl_stack_pointer_register_write(rvbl_pointer_t value);

/// == Function `rvbl_global_pointer_register_read`
/// Returns the current value of the `gp` (`x3`) register.
///
/// === Return value
/// `rvbl_pointer_t`:: Current value of the `gp` (`x3`) register.
rvbl_pointer_t rvbl_global_pointer_register_read(void);

/// == Function `rvbl_global_pointer_register_write`
/// Sets the current value of the `gp` (`x3`) register.
///
/// === Parameters
/// `rvbl_pointer_t` `value`:: New value of the `gp` (`x3`) register.
void rvbl_global_pointer_register_write(rvbl_pointer_t value);

/// == Function `rvbl_system_call_0`
/// Performs an indexed environment call (`ecall` instruction) with zero
/// parameters.
///
/// === Parameters
/// `rvbl_uword_t` `index`:: Environment call index.
///
/// === Return value
/// `rvbl_uword_t`:: Result of the environment call.
rvbl_uword_t rvbl_system_call_0(rvbl_uword_t index);

/// == Function `rvbl_system_call_1`
/// Performs an indexed environment call (`ecall` instruction) with one
/// parameters.
///
/// === Parameters
/// `rvbl_uword_t` `index`:: Environment call index.
/// `rvbl_uword_t`:: First environment call parameter.
///
/// === Return value
/// `rvbl_uword_t`:: Result of the environment call.
rvbl_uword_t rvbl_system_call_1(rvbl_uword_t index, rvbl_uword_t);

/// == Function `rvbl_system_call_2`
/// Performs an indexed environment call (`ecall` instruction) with two
/// parameters.
///
/// === Parameters
/// `rvbl_uword_t` `index`:: Environment call index.
/// `rvbl_uword_t`:: First environment call parameter.
/// `rvbl_uword_t`:: Second environment call parameter.
///
/// === Return value
/// `rvbl_uword_t`:: Result of the environment call.
rvbl_uword_t rvbl_system_call_2(rvbl_uword_t index, rvbl_uword_t, rvbl_uword_t);

/// == Function `rvbl_system_call_3`
/// Performs an indexed environment call (`ecall` instruction) with threee
/// parameters.
///
/// === Parameters
/// `rvbl_uword_t` `index`:: Environment call index.
/// `rvbl_uword_t`:: First environment call parameter.
/// `rvbl_uword_t`:: Second environment call parameter.
/// `rvbl_uword_t`:: Thirs environment call parameter.
///
/// === Return value
/// `rvbl_uword_t`:: Result of the environment call.
rvbl_uword_t rvbl_system_call_3(rvbl_uword_t index, rvbl_uword_t, rvbl_uword_t, rvbl_uword_t);

#endif