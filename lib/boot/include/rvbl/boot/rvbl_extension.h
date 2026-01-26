#ifndef RVBL_EXTENSION_H
#define RVBL_EXTENSION_H

#include "rvbl/core/rvbl_types.h"
#include "rvbl/machine/rvbl_machine.h"

/// == Function `rvbl_extension_present`
/// Returns whether a particular RISC-V extension is available.
///
/// === Parameters
/// `misa_extensions`:: RISC-V extension to check.
///
/// === Return value
/// `rvbl_bool_t`:: Whether given extension is available.
rvbl_bool_t rvbl_extension_present(misa_extensions_values);

#endif
