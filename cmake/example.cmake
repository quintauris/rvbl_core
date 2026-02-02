add_executable(${example_name} ${example_sources})
target_link_options(${example_name} PRIVATE ${rvbl_linker_nostdlib})
target_link_libraries(${example_name} PRIVATE rvbl_c rvbl_machine rvbl_semihost
                                              ${example_extra_libraries})

# cmake-lint: disable=C0307
set_target_properties(
  ${example_name} PROPERTIES LINKER_SCRIPT
                             "${CMAKE_SOURCE_DIR}/linker/${rvbl_linker_id}.ld")

add_custom_command(
  TARGET ${example_name}
  POST_BUILD
  COMMAND ${OBJCOPY} $<TARGET_FILE:${example_name}> ${rvbl_elf_copy_parameters}
          ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin
  BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin
  COMMENT "Converting $<TARGET_FILE:${example_name}> to binary image...")

if(example_truncate_size)
  find_program(TRUNCATE NAMES truncate)

  if(TRUNCATE)
    add_custom_command(
      TARGET ${example_name}
      POST_BUILD
      COMMAND ${TRUNCATE} -s ${example_truncate_size}
              ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin
      COMMENT "Resizing ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin ...")
  else()
    message(
      WARNING "No \"truncate\" executable found, ${example_name}.bin shall be \
manually resized to ${example_truncate_size}.")
  endif()
endif()

if(example_test_timeout AND example_expected_output)
  get_property(rvbl_rave_local GLOBAL PROPERTY rvbl_rave_local)
  get_property(rvbl_toolchain_moniker GLOBAL PROPERTY rvbl_toolchain_moniker)
  get_property(rvbl_machine_moniker GLOBAL PROPERTY rvbl_machine_moniker)
  get_property(rvbl_runners GLOBAL PROPERTY rvbl_runners)

  if("jrun" IN_LIST rvbl_runners)
    add_test(
      NAME jrun.example.${example_name}.elf
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner segger.jrun --dump-file
        ${CMAKE_CURRENT_BINARY_DIR}/:tt --file
        ${CMAKE_CURRENT_BINARY_DIR}/${example_name}${CMAKE_EXECUTABLE_SUFFIX_C})
    set_tests_properties(
      jrun.example.${example_name}.elf PROPERTIES PASS_REGULAR_EXPRESSION
                                                  ${example_expected_output})
    set_tests_properties(jrun.example.${example_name}.elf
                         PROPERTIES TIMEOUT ${example_test_timeout})
  endif()

  if("qemu" IN_LIST rvbl_runners)
    add_test(
      NAME qemu.example.${example_name}.elf
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner qemu.system32-elf --file
        ${CMAKE_CURRENT_BINARY_DIR}/${example_name}${CMAKE_EXECUTABLE_SUFFIX_C})
    set_tests_properties(
      qemu.example.${example_name}.elf PROPERTIES PASS_REGULAR_EXPRESSION
                                                  ${example_expected_output})
    set_tests_properties(qemu.example.${example_name}.elf
                         PROPERTIES TIMEOUT ${example_test_timeout})

    add_test(
      NAME qemu.example.${example_name}.bin
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner qemu.system32-bin --file
        ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin)
    set_tests_properties(
      qemu.example.${example_name}.bin PROPERTIES PASS_REGULAR_EXPRESSION
                                                  ${example_expected_output})
    set_tests_properties(qemu.example.${example_name}.bin
                         PROPERTIES TIMEOUT ${example_test_timeout})
  endif()

  if("vdk" IN_LIST rvbl_runners)
    add_test(
      NAME vdk.example.${example_name}.elf
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner quintauris_internal.vdk-elf --file
        ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.${CMAKE_EXECUTABLE_SUFFIX_C}
    )
    set_tests_properties(
      vdk.example.${example_name}.elf PROPERTIES PASS_REGULAR_EXPRESSION
                                                 ${example_expected_output})
    set_tests_properties(vdk.example.${example_name}.elf
                         PROPERTIES TIMEOUT ${example_test_timeout})

    add_test(
      NAME vdk.example.${example_name}.bin
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner quintauris_internal.vdk-bin --file
        ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin)
    set_tests_properties(
      vdk.example.${example_name}.bin PROPERTIES PASS_REGULAR_EXPRESSION
                                                 ${example_expected_output})
    set_tests_properties(vdk.example.${example_name}.bin
                         PROPERTIES TIMEOUT ${example_test_timeout})
  endif()

  if("t32" IN_LIST rvbl_runners)
    add_test(
      NAME t32.example.${example_name}.elf
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner lauterbach.t32 --delay 8 --dump-file
        output.txt --file
        ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C}
    )
    set_tests_properties(
      t32.example.${example_name}.elf PROPERTIES PASS_REGULAR_EXPRESSION
                                                 ${example_expected_output})
    set_tests_properties(t32.example.${example_name}.elf
                         PROPERTIES TIMEOUT ${example_test_timeout})
  endif()
endif()

if(example_build_for_flash)
  set(example_name "${example_name}_flash")
  add_executable(${example_name} ${example_sources})
  target_link_options(${example_name} PRIVATE ${rvbl_linker_nostdlib})
  target_link_libraries(
    ${example_name} PRIVATE rvbl_c rvbl_machine rvbl_semihost
                            ${example_extra_libraries})
  target_compile_definitions(${example_name} PRIVATE RVBL_BUILD_FLASH)
  set_target_properties(
    ${example_name}
    PROPERTIES LINKER_SCRIPT
               "${CMAKE_SOURCE_DIR}/linker/${rvbl_linker_id}_flash.ld")

  add_custom_command(
    TARGET ${example_name}
    POST_BUILD
    COMMAND
      ${OBJCOPY} $<TARGET_FILE:${example_name}> ${rvbl_elf_copy_parameters}
      ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin
    BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin
    COMMENT "Converting $<TARGET_FILE:${example_name}> to binary image...")
endif()
