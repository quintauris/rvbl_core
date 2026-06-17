/*
 * Copyright 2026 Quintauris GmbH
 * Licensed under the Apache License, Version 2.0 (the "License").
 * https://www.apache.org/licenses/LICENSE-2.0
 */

#include "rvbl/boot/rvbl_boot.h"

extern const rvbl_boot_step boot_steps[];
rvbl_uword_t rvbl_boot_barrier;

__attribute__((section(".text.start"))) void rvbl_boot(void)
{
    register const rvbl_boot_step *step = boot_steps;

    for (step = boot_steps; step->action != rvbl_boot_action_finish; ++step) {
        switch (step->action) {
        case rvbl_boot_action_fill: {
            rvbl_uint8_t *a = step->parameters.fill.begin_adddress;
            rvbl_uint8_t *b = step->parameters.fill.end_adddress;

            for (; a != b; ++a) {
                *a = step->parameters.fill.value;
            }
        } break;
        case rvbl_boot_action_copy: {
            const rvbl_uint8_t *a = step->parameters.copy.begin_adddress;
            const rvbl_uint8_t *b = step->parameters.copy.end_adddress;
            rvbl_uint8_t *c = step->parameters.copy.target_address;

            for (; a != b; ++a, ++c) {
                *c = *a;
            }
        } break;
        case rvbl_boot_action_call:
            step->parameters.call.callout(
                step->parameters.call.parameter1, step->parameters.call.parameter2
            );
            break;
        default:
            break;
        }
    }
}
