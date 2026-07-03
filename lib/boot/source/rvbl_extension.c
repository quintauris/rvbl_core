/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/boot/rvbl_extension.h"

rvbl_bool_t rvbl_extension_present(const misa_extensions_values extension)
{
    return (rvbl_misa_read() & (1UL << extension)) ? rvbl_true : rvbl_false;
}
