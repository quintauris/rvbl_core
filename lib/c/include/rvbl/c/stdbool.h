#ifndef RVBL_STDBOOL_H
#define RVBL_STDBOOL_H

/// == <stdbool.h>

#include "rvbl/type/rvbl_types.h"

/// === Type `bool`
/// Convenience macro, expands to `rvbl_bool_t`.
typedef rvbl_bool_t bool;

/// === Macro `true`
/// Expands to `rvbl_true`.
#define true rvbl_true

/// === Macro `false`
/// Expands to `rvbl_false`.
#define false rvbl_false

#endif
