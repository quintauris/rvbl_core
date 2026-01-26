define block GP_STACK with size = STACK_SIZE, alignment = 8 { };
define block SP_STACK with size = STACK_SIZE, alignment = 8 { };
define block IR_STACK with size = STACK_SIZE, alignment = 8 { };
define block HEAP with size = HEAP_SIZE, alignment = 8 { };

/* Handle initialization */
do not initialize {
    section .text.start,
    section .rodata.start,
    section .bss
};

initialize manually {
    section .text,
    section .rodata,
    section .data,
};

define block ROM_CONTENT {
    section .text_init,
    section text_asm_init,
    section .rodata_init,
    section .data_init,
};

define block RAM_CONTENT {
    section .text,
    section .rodata,
    section .data,
    section .bss
};

define image symbol text_begin_dram = ADDR(.text);
define image symbol text_end_dram = text_begin_dram + SIZE(.text);
define image symbol text_begin_flash = ADDR(.text_init);
define image symbol text_end_flash = text_begin_flash + SIZE(.text_init);

define image symbol rodata_begin_dram = ADDR(.rodata);
define image symbol rodata_end_dram = rodata_begin_dram + SIZE(.rodata);
define image symbol rodata_begin_flash = ADDR(.rodata_init);
define image symbol rodata_end_flash = rodata_begin_flash + SIZE(.rodata_init);

define image symbol data_begin_dram = ADDR(.data);
define image symbol data_end_dram = data_begin_dram + SIZE(.data);
define image symbol data_begin_flash = ADDR(.data_init);
define image symbol data_end_flash = data_begin_flash + SIZE(.data_init);

define image symbol bss_begin = ADDR(.bss);
define image symbol bss_end = bss_begin + SIZE(.bss);

define image symbol general_purpose_stack_top = ADDR(GP_STACK) + STACK_SIZE;
define image symbol interrupt_stack_top = ADDR(IR_STACK) + STACK_SIZE;
define image symbol supervisor_stack_top = ADDR(SP_STACK) + STACK_SIZE;
