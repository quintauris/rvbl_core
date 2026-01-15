.global rvbl_semihost_interface

.section .text

rvbl_semihost_interface:
    slli    x0, x0, 0x1f
    ebreak
    srai    x0, x0,  0x7
    ret

.end
