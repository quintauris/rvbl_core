section_setup ::linear
{
  stack "general_purpose_stack" (min_size=4K, grows=high_to_low, align=8, fixed);
  stack "supervisor_stack" (min_size=4K, grows=high_to_low, align=8, fixed);
  stack "interrupt_stack" (min_size=4K, grows=high_to_low, align=8, fixed);
}

section_layout ::linear
{
  group boot_start (ordered, run_addr=mem:ROM)
  {
    select ".text.start";
    select ".text.rvbl_boot.rvbl_boot";
    select ".rodata.rvbl_configuration.boot_steps";
  }

  group program (contiguous, load_addr=mem:FLASH, ordered, fill)
  {
    group program_text
    {
      select ".text*";
    }

    "text_begin_flash" := addressof(group:program_text);
    "text_end_flash" := addressof(group:program_text) + sizeof(group:program_text);

    group program_rodata
    {
      select ".rodata*";
    }

    "rodata_begin_flash" := addressof(group:program_rodata);
    "rodata_end_flash" := addressof(group:program_rodata) + sizeof(group:program_rodata);

    group program_data
    {
      select ".data*";
    }

    "data_begin_flash" := addressof(group:program_data);
    "data_end_flash" := addressof(group:program_data) + sizeof(group:program_data);
  }

  group scratch (copy, contiguous, run_addr=mem:RAM, ordered, fill)
  {
    group scratch_text
    {
      select ".text*";
    }

    "text_begin_dram" := addressof(group:scratch_text);
    "text_end_dram" := addressof(group:scratch_text) + sizeof(group:scratch_text);

    group scratch_rodata
    {

      select ".rodata*";
    }

    "rodata_begin_dram" := addressof(group:scratch_rodata);
    "rodata_end_dram" := addressof(group:scratch_rodata) + sizeof(group:scratch_rodata);

    group scratch_data
    {
      select ".data*";
    }

    "data_begin_dram" := addressof(group:scratch_data);
    "data_end_dram" := addressof(group:scratch_data) + sizeof(group:scratch_data);
  }

  group bss (contiguous, run_addr=mem:RAM)
  {
    select ".bss*";
    "bss_begin" := "_lc_gb_bss";
    "bss_end" := "_lc_ge_bss";
  }

  group stacks (contiguous, run_addr=mem:RAM)
  {
    stack "general_purpose_stack";
    stack "supervisor_stack";
    stack "interrupt_stack";

    "general_purpose_stack_top" := "_lc_ub_general_purpose_stack";
    "general_purpose_stack_begin" := "_lc_ue_general_purpose_stack";
    "supervisor_stack_top" := "_lc_ub_supervisor_stack";
    "supervisor_stack_begin" := "_lc_ue_supervisor_stack";
    "interrupt_stack_top" := "_lc_ub_interrupt_stack";
    "interrupt_stack_begin" := "_lc_ue_interrupt_stack";
  }
}
