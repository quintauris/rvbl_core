function(rvbl_setup_generators)
    find_program(
        DOC_EXTRACT_GENERIC
        NAMES extract_generic 
        HINTS "${host_tools}/doc"
        REQUIRED
        NO_DEFAULT_PATH
    )

    find_program(
        ASCIIDOCTOR
        NAMES asciidoctor 
        REQUIRED
    )

    find_program(
        MACHINE_GENERATOR
        NAMES generator 
        HINTS "${host_tools}/generator"
        REQUIRED
        NO_DEFAULT_PATH
    )
endfunction()

function(rvbl_run_generators)
    rvbl_setup_generators()

    set(vendor_path "${CMAKE_SOURCE_DIR}/../../..")
    set(generated_path "${CMAKE_CURRENT_SOURCE_DIR}/generated")
    set(documentation_path "${CMAKE_CURRENT_SOURCE_DIR}/generated/doc")
    file(MAKE_DIRECTORY ${generated_path} ${documentation_path})

    execute_process(
        COMMAND "${MACHINE_GENERATOR}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json" "${vendor_path}/quintauris_community/meta/machine.schema.json" "${vendor_path}" --dependencies
        OUTPUT_VARIABLE model_files
        #COMMAND_ECHO STDOUT
        #ECHO_OUTPUT_VARIABLE
    )
    string(STRIP "${model_files}" model_files)
    list(TRANSFORM model_files PREPEND "${vendor_path}/")
    list(APPEND model_files "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json")
    set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS "${model_files}")

    execute_process(
        COMMAND "${MACHINE_GENERATOR}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json" "${vendor_path}/quintauris_community/meta/machine.schema.json" "${vendor_path}" --dry
        WORKING_DIRECTORY "${generated_path}"
        OUTPUT_VARIABLE generated_files
        #COMMAND_ECHO STDOUT
        #ECHO_OUTPUT_VARIABLE
    )
    string(STRIP "${generated_files}" generated_files)
    list(TRANSFORM generated_files PREPEND "${generated_path}/")

    # C files
    add_custom_command(
        OUTPUT ${generated_files}
        COMMAND "${MACHINE_GENERATOR}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json" "${vendor_path}/quintauris_community/meta/machine.schema.json" "${vendor_path}"
        DEPENDS ${model_files}
        WORKING_DIRECTORY ${generated_path}
        COMMENT "Generating machine headers from ${CMAKE_CURRENT_SOURCE_DIR}/model/model.json ..."
        #COMMAND_ECHO STDOUT
        #ECHO_OUTPUT_VARIABLE
    )

    # Linker files
    add_custom_command(
        OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/memory.GNU.ld"
        COMMAND "${MACHINE_GENERATOR}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json" "${vendor_path}/quintauris_community/meta/machine.schema.json" "${vendor_path}" --linker
        MAIN_DEPENDENCY "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json"
        DEPENDS "${model_files}"
        WORKING_DIRECTORY ${generated_path}
        BYPRODUCTS "${CMAKE_CURRENT_BINARY_DIR}/memory.GNU.ld" "${CMAKE_CURRENT_BINARY_DIR}/memory.Tasking.ld" "${CMAKE_CURRENT_BINARY_DIR}/memory.IAR.ld"
        COMMENT "Generating linker files from ${CMAKE_CURRENT_SOURCE_DIR}/model/model.json ..."
    )
    add_custom_target(linker_files ALL DEPENDS "${CMAKE_CURRENT_BINARY_DIR}/memory.GNU.ld")

    # Model documentation
    add_custom_command(
        OUTPUT "${documentation_path}/model.html"
        COMMAND "${MACHINE_GENERATOR}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json" "${vendor_path}/quintauris_community/meta/machine.schema.json" "${vendor_path}" --documentation
        COMMAND "${ASCIIDOCTOR}" -o "${documentation_path}/model.html" "${documentation_path}/model.adoc"
        WORKING_DIRECTORY "${documentation_path}"
        MAIN_DEPENDENCY "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json"
        DEPENDS "${model_files}"
        BYPRODUCTS "${documentation_path}/model.adoc"
        COMMENT "Generating model documentation from ${CMAKE_CURRENT_SOURCE_DIR}/model/model.json ..."
    )
    add_custom_target(model_doc ALL DEPENDS "${documentation_path}/model.html")

    # Machine library documentation
    set(rvbl_machine_sources ${generated_files})
    set(machine_list "")
    set(counter "0")

    foreach(source ${rvbl_machine_sources})
        get_filename_component(source_name_we ${source} NAME_WE)
        set(adoc_output "${documentation_path}/${source_name_we}.adoc")

        if (${adoc_output} IN_LIST machine_list)
            string(APPEND source_name_we "_${counter}")
            set(adoc_output "${documentation_path}/${source_name_we}.adoc")
            math(EXPR counter "${counter} + 1")
        endif()

        add_custom_command(
            OUTPUT ${adoc_output}
            COMMAND ${DOC_EXTRACT_GENERIC} ${source} ${adoc_output}
            DEPENDS ${source}
        )
        list(APPEND machine_list ${adoc_output})
    endforeach()

    add_custom_command(
        OUTPUT "${documentation_path}/rvbl/machine/rvbl_machine.html"
        COMMAND "${ASCIIDOCTOR}" -o "${documentation_path}/rvbl/machine/rvbl_machine.html" "${documentation_path}/rvbl_machine.adoc" 
        DEPENDS ${machine_list}
        COMMENT "Generating source documentation from ${CMAKE_CURRENT_SOURCE_DIR}/model/model.json ..."
    )
    add_custom_target(machine_doc ALL DEPENDS "${documentation_path}/rvbl/machine/rvbl_machine.html")

    return(PROPAGATE generated_files)
endfunction()

function(rvbl_documentation module)
    set(generated_path "${CMAKE_CURRENT_SOURCE_DIR}/generated")
    set(documentation_path "${CMAKE_CURRENT_SOURCE_DIR}/generated")
    file(MAKE_DIRECTORY ${generated_path} ${documentation_path})
    
    get_target_property(sources rvbl_${module} SOURCES)
    add_custom_command(
        OUTPUT "${documentation_path}/${module}.html"
        COMMAND "${DOC_EXTRACT_GENERIC}" ${sources} "${documentation_path}/${module}.adoc"
        COMMAND ${ASCIIDOCTOR} -o "${documentation_path}/${module}.html" "${documentation_path}/${module}.adoc" 
        DEPENDS ${sources}
        BYPRODUCTS "${documentation_path}/${module}.adoc"
        COMMENT "Generating documentation for ${module} ..."
    )
    add_custom_target(${module}_doc ALL DEPENDS "${documentation_path}/${module}.html")
endfunction()

function(rvbl_interface_library)
    set(options "")
    set(oneValueArgs NAME)
    set(multiValueArgs SOURCES INCLUDES LIBRARIES)
    cmake_parse_arguments(
        PARSE_ARGV 0 arg
        "${options}" "${oneValueArgs}" "${multiValueArgs}"
    )

    add_library(rvbl_${arg_NAME} INTERFACE ${arg_SOURCES})
    target_include_directories(rvbl_${arg_NAME} INTERFACE ${arg_INCLUDES})
    target_link_libraries(rvbl_${arg_NAME} INTERFACE $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)

    rvbl_documentation(${arg_NAME})
endfunction()

function(rvbl_static_library)
    set(options "")
    set(oneValueArgs NAME)
    set(multiValueArgs SOURCES INCLUDES LIBRARIES)
    cmake_parse_arguments(
        PARSE_ARGV 0 arg
        "${options}" "${oneValueArgs}" "${multiValueArgs}"
    )

    add_library(rvbl_${arg_NAME} STATIC ${arg_SOURCES})
    target_include_directories(rvbl_${arg_NAME} PUBLIC ${arg_INCLUDES})
    target_link_libraries(rvbl_${arg_NAME} PUBLIC $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)

    rvbl_documentation(${arg_NAME})
endfunction()

function(rvbl_test)
    set(options "")
    set(oneValueArgs NAME)
    set(multiValueArgs LIBRARIES DEVICES)
    cmake_parse_arguments(
        PARSE_ARGV 0 arg
        "${options}" "${oneValueArgs}" "${multiValueArgs}"
    )

    get_property(qemu_machine_settings GLOBAL PROPERTY qemu_machine_settings)
    get_property(jrun_settings GLOBAL PROPERTY jrun_settings)
    get_property(vdk_settings GLOBAL PROPERTY vdk_settings)
    get_property(trace32_settings GLOBAL PROPERTY trace32_settings)
    
    add_executable(    
        test_${arg_NAME} 
        "${init}"
        "${CMAKE_CURRENT_SOURCE_DIR}/tests/${arg_NAME}.c"
    )
    target_link_options(test_${arg_NAME} PRIVATE ${rvbl_linker_nostdlib})
    target_link_libraries(test_${arg_NAME} PRIVATE rvbl_test $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)
    set_target_properties(
        test_${arg_NAME}
        PROPERTIES
            LINKER_SCRIPT "${CMAKE_SOURCE_DIR}/linker/${rvbl_linker_id}.ld"
            LINKER_INCLUDE_PATH_1 "${CMAKE_SOURCE_DIR}/../../../base/linker"
            LINKER_INCLUDE_PATH_2 "${CMAKE_SOURCE_DIR}/generated"
    )

    if (qemu_machine_settings)
        add_test(
            NAME qemu.${arg_NAME}
            COMMAND ${CMAKE_SOURCE_DIR}/../../../base/lib/test/script/driver.sh qemu ${CMAKE_PROJECT_NAME} ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C} ${qemu_machine_settings}
        )
        set_tests_properties(qemu.${arg_NAME} PROPERTIES SKIP_RETURN_CODE 2)
    endif()

    if (jrun_settings)
        add_test(
            NAME jrun.${arg_NAME}
            COMMAND ${CMAKE_SOURCE_DIR}/../../../base/lib/test/script/driver.sh jrun ${CMAKE_PROJECT_NAME} ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C} ${jrun_settings}
        )
        set_tests_properties(jrun.${arg_NAME} PROPERTIES SKIP_RETURN_CODE 2)
    endif()

    if (openocd_settings)
        add_test(
            NAME openocd.${arg_NAME}
            COMMAND ${CMAKE_SOURCE_DIR}/../../../base/lib/test/script/driver.sh openocd ${CMAKE_PROJECT_NAME} ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C} ${openocd_settings}
        )
        set_tests_properties(
            openocd.${arg_NAME}
            PROPERTIES
                PASS_REGULAR_EXPRESSION "PASS"
                FAIL_REGULAR_EXPRESSION "FAIL"
                SKIP_REGULAR_EXPRESSION "SKIP"
        )
    endif()

    if (vdk_settings)
        add_test(
            NAME vdk.${arg_NAME}
            COMMAND ${CMAKE_SOURCE_DIR}/../../../base/lib/test/script/driver.sh vdk ${CMAKE_PROJECT_NAME} ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C} ${vdk_settings}
        )
        set_tests_properties(
            vdk.${arg_NAME}
            PROPERTIES
                PASS_REGULAR_EXPRESSION "PASS"
                FAIL_REGULAR_EXPRESSION "FAIL"
                SKIP_REGULAR_EXPRESSION "SKIP"
                TIMEOUT 120
        )
    endif()

    if (trace32_settings)
        add_test(
            NAME t32.${arg_NAME}
            COMMAND ${CMAKE_SOURCE_DIR}/../../../base/lib/test/script/driver.sh t32 ${CMAKE_PROJECT_NAME} ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C} ${trace32_settings}
        )
        set_tests_properties(
            t32.${arg_NAME}
            PROPERTIES
                PASS_REGULAR_EXPRESSION "PASS"
                FAIL_REGULAR_EXPRESSION "FAIL"
                SKIP_REGULAR_EXPRESSION "SKIP"
        )
    endif()
endfunction()
