    public rvbl_test_cause_store_fault
    public rvbl_hang

    section `.text`:CODE

rvbl_test_cause_store_fault:
    li a0, -3
    sw zero, 0(a0)
    ret

rvbl_hang:
    wfi
    j rvbl_hang

    end
