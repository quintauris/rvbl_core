.global rvbl_thread_pointer_register_write
.global rvbl_thread_pointer_register_read
.global rvbl_stack_pointer_register_write
.global rvbl_stack_pointer_register_read
.global rvbl_global_pointer_register_read
.global rvbl_global_pointer_register_write
.global rvbl_system_call_0
.global rvbl_system_call_1
.global rvbl_system_call_2
.global rvbl_system_call_3

.section .text

rvbl_thread_pointer_register_read:
    mv a0, tp
    ret

rvbl_thread_pointer_register_write:
    mv tp, a0
    ret

rvbl_stack_pointer_register_read:
    mv a0, sp
    ret

rvbl_stack_pointer_register_write:
    mv sp, a0
    ret

rvbl_global_pointer_register_read:
    mv a0, gp
    ret

rvbl_global_pointer_register_write:
    mv gp, a0
    ret

rvbl_system_call_3:
rvbl_system_call_2:
rvbl_system_call_1:
rvbl_system_call_0:
    ecall
    ret

.end
