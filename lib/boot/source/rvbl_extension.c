/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/boot/rvbl_extension.h"
#include "rvbl/hardware/rvbl_hardware.h"

rvbl_bool_t
rvbl_extension_present(const enum rvbl_riscv_hart_privileged_misa_extensions_t extension)
{
    return (RVBL_REGISTER_READ(riscv_hart, privileged, &rvbl_riscv_hart_instance_0, misa)
                .f.extensions &
            (1UL << extension))
               ? rvbl_true
               : rvbl_false;
}
