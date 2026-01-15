#ifndef RVBL_BOOT_H
#define RVBL_BOOT_H

#include "rvbl/core/rvbl_types.h"

typedef enum rvbl_boot_action
{
    rvbl_boot_action_fill,
    rvbl_boot_action_copy,
    rvbl_boot_action_call,
    rvbl_boot_action_finish
} rvbl_boot_action;

typedef struct rvbl_boot_action_fill_parameters
{
    rvbl_pointer_t begin_adddress;
    rvbl_pointer_t end_adddress;
    rvbl_uword_t value;
} rvbl_boot_action_fill_paramters;

typedef struct rvbl_boot_action_copy_parameters
{
    rvbl_pointer_t begin_adddress;
    rvbl_pointer_t end_adddress;
    rvbl_pointer_t target_address;
} rvbl_boot_action_copy_paramters;

typedef void (*rvbl_boot_action_callout)(rvbl_uword_t, rvbl_uword_t);

typedef struct rvbl_boot_action_call_parameters
{
    rvbl_boot_action_callout callout;
    rvbl_uword_t parameter1;
    rvbl_uword_t parameter2;
} rvbl_boot_action_call_paramters;

typedef union rvbl_boot_action_parameters
{
    struct rvbl_boot_action_fill_parameters fill;
    struct rvbl_boot_action_copy_parameters copy;
    struct rvbl_boot_action_call_parameters call;
} rvbl_boot_action_parameters;

typedef struct rvbl_boot_step
{
    rvbl_boot_action action;
    rvbl_boot_action_parameters parameters;
} rvbl_boot_step;

void rvbl_boot(void);

#endif