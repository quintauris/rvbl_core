add_executable(${example_name} ${example_sources})
target_link_options(${example_name} PRIVATE ${rvbl_linker_nostdlib})
target_link_libraries(${example_name} PRIVATE rvbl_c rvbl_machine rvbl_semihost ${example_extra_libraries})
set_target_properties(
    ${example_name}
    PROPERTIES
        LINKER_SCRIPT "${CMAKE_SOURCE_DIR}/linker/${rvbl_linker_id}.ld"
        LINKER_INCLUDE_PATH_1 "${CMAKE_SOURCE_DIR}/../../../base/linker"
        LINKER_INCLUDE_PATH_2 "${CMAKE_SOURCE_DIR}/generated"
)

add_custom_command(
    TARGET ${example_name}
    POST_BUILD
    COMMAND ${OBJCOPY} $<TARGET_FILE:${example_name}> ${rvbl_elf_copy_parameters} ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin
    BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin
    COMMENT "Converting $<TARGET_FILE:${example_name}> to binary image..."
)

if (example_truncate_size)
    find_program(
        TRUNCATE 
        NAMES truncate
    )

    if (TRUNCATE)
        add_custom_command(
            TARGET ${example_name}
            POST_BUILD
            COMMAND ${TRUNCATE} -s ${example_truncate_size} ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin 
            COMMENT "Resizing ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin ..."
        )
    else()
        message(WARNING "No \"truncate\" executable found, ${example_name}.bin shall be manually resized to ${example_truncate_size}.")
    endif()
endif()

if (example_test_timeout AND example_expected_output)
    get_property(qemu_machine_settings GLOBAL PROPERTY qemu_machine_settings)

    if (jrun_settings)
        add_test(
            NAME jrun.example.${example_name}.elf
            COMMAND ${CMAKE_SOURCE_DIR}/lib/test/script/driver.sh jrun ${machine} ${CMAKE_CURRENT_BINARY_DIR}/${example_name}${CMAKE_EXECUTABLE_SUFFIX_C} ${jrun_settings}
        )
        set_tests_properties(jrun.example.${example_name}.elf PROPERTIES PASS_REGULAR_EXPRESSION ${example_expected_output})
        set_tests_properties(jrun.example.${example_name}.elf PROPERTIES TIMEOUT ${example_test_timeout})
    endif()

    if (qemu_machine_settings)
        add_test(
            NAME qemu.example.${example_name}.elf
            COMMAND ${CMAKE_SOURCE_DIR}/../../../base/lib/test/script/driver.sh qemu ${CMAKE_PROJECT_NAME} ${CMAKE_CURRENT_BINARY_DIR}/${example_name}${CMAKE_EXECUTABLE_SUFFIX_C} ${qemu_machine_settings}
        )
        set_tests_properties(qemu.example.${example_name}.elf PROPERTIES PASS_REGULAR_EXPRESSION ${example_expected_output})
        set_tests_properties(qemu.example.${example_name}.elf PROPERTIES TIMEOUT ${example_test_timeout})

        add_test(
            NAME qemu.example.${example_name}.bin
            COMMAND ${CMAKE_SOURCE_DIR}/../../../base/lib/test/script/driver.sh qemu.bin ${CMAKE_PROJECT_NAME} ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin ${qemu_machine_settings}
        )
        set_tests_properties(qemu.example.${example_name}.bin PROPERTIES PASS_REGULAR_EXPRESSION ${example_expected_output})
        set_tests_properties(qemu.example.${example_name}.bin PROPERTIES TIMEOUT ${example_test_timeout})
    endif()

    if (vdk_settings)
        add_test(
            NAME vdk.example.${example_name}.elf
            COMMAND ${CMAKE_SOURCE_DIR}/lib/test/script/driver.sh vdk ${machine} ${CMAKE_CURRENT_BINARY_DIR}/${example_name}${CMAKE_EXECUTABLE_SUFFIX_C} ${vdk_settings}
        )
        set_tests_properties(vdk.example.${example_name}.elf PROPERTIES PASS_REGULAR_EXPRESSION ${example_expected_output})
        set_tests_properties(vdk.example.${example_name}.elf PROPERTIES TIMEOUT ${example_test_timeout})

        add_test(
            NAME vdk.example.${example_name}.bin
            COMMAND ${CMAKE_SOURCE_DIR}/lib/test/script/driver.sh vdk.bin ${machine} ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin ${vdk_settings}
        )
        set_tests_properties(vdk.example.${example_name}.bin PROPERTIES PASS_REGULAR_EXPRESSION ${example_expected_output})
        set_tests_properties(vdk.example.${example_name}.bin PROPERTIES TIMEOUT ${example_test_timeout})
    endif()

    if (trace32_settings)
        add_test(
            NAME t32.example.${example_name}.elf
            COMMAND ${CMAKE_SOURCE_DIR}/lib/test/script/driver.sh t32 ${machine} ${CMAKE_CURRENT_BINARY_DIR}/${example_name}${CMAKE_EXECUTABLE_SUFFIX_C} ${trace32_settings}
        )
        set_tests_properties(t32.example.${example_name}.elf PROPERTIES PASS_REGULAR_EXPRESSION ${example_expected_output})
        set_tests_properties(t32.example.${example_name}.elf PROPERTIES TIMEOUT ${example_test_timeout})
    endif()
endif()

if (example_build_for_flash)
    set(example_name "${example_name}_flash")
    add_executable(${example_name} ${example_sources})
    target_link_options(${example_name} PRIVATE ${rvbl_linker_nostdlib})
    target_link_libraries(${example_name} PRIVATE rvbl_c rvbl_machine rvbl_semihost ${example_extra_libraries})
    target_compile_definitions(${example_name} PRIVATE RVBL_BUILD_FLASH)
    set_target_properties(
        ${example_name}
        PROPERTIES
            LINKER_SCRIPT "${CMAKE_SOURCE_DIR}/linker/${rvbl_linker_id}.ld"
            LINKER_INCLUDE_PATH_1 "${CMAKE_SOURCE_DIR}/../../../base/linker"
            LINKER_INCLUDE_PATH_2 "${CMAKE_SOURCE_DIR}/generated"
    )

    add_custom_command(
        TARGET ${example_name}
        POST_BUILD
        COMMAND ${OBJCOPY} $<TARGET_FILE:${example_name}> ${rvbl_elf_copy_parameters} ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin
        BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/${example_name}.bin
        COMMENT "Converting $<TARGET_FILE:${example_name}> to binary image..."
    )
endif()