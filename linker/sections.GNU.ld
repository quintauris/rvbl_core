SECTIONS {
  .text.start : { *(.text.start) } > ROM 
  .rodata.start : { *(.rodata.start) } > ROM
  
  .text : { 
    . = ALIGN(8);
    text_begin_dram = .;
    text_begin_flash = LOADADDR(.text);
    *(.text*)
    . = ALIGN(8);
    text_end_dram = .;
    text_end_flash = LOADADDR(.text) + (text_end_dram - text_begin_dram);
  } > RAM AT > FLASH

  .rodata : {
    . = ALIGN(8);
    rodata_begin_dram = .;
    rodata_begin_flash = LOADADDR(.rodata);
    *(.rodata*)
    . = ALIGN(8);
    rodata_end_dram = .;
    rodata_end_flash = LOADADDR(.rodata) + (rodata_end_dram - rodata_begin_dram);
  } > RAM AT > FLASH
  
  .data : { 
    . = ALIGN(8);
    data_begin_dram = .;
    data_begin_flash = LOADADDR(.data);
    *(.data*)
    . = ALIGN(8);
    data_end_dram = .;
    data_end_flash = LOADADDR(.data) + (data_end_dram - data_begin_dram);
  } > RAM AT > FLASH

  .bss : { 
    . = ALIGN(8);
    bss_begin = .;
    *(.bss*)
    . = ALIGN(8);
    bss_end = .;

    general_purpose_stack_begin = .;
    . = . + 4K;
    . = ALIGN(8);
    general_purpose_stack_top = .;

    supervisor_stack_begin = .;
    . = . + 4K;
    . = ALIGN(8);
    supervisor_stack_top = .;

    interrupt_stack_begin = .;
    . = . + 4K;
    . = ALIGN(8);
    interrupt_stack_top = .;

    heap_begin = .;
    . = . + 4K;
    . = ALIGN(8);
    heap_top = .;
  } > RAM
}
