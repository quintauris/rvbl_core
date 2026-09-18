#[[
  Copyright 2026 Quintauris GmbH
  Licensed under the Apache License, Version 2.0 (the "License").
  https://www.apache.org/licenses/LICENSE-2.0
]]

function(rvbl_machine)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "" "LIBRARIES;RUNNERS;BOOTLOADER")

  if(DEFINED ENV{RVBL_ROOT})
    set(rvbl_root $ENV{RVBL_ROOT})
  else()
    message(FATAL_ERROR "Environment variable RVBL_ROOT not defined, please \
source \"bootstrap.sh\".")
  endif()

  if(DEFINED ENV{RVBL_TOOL})
    set(rvbl_tool $ENV{RVBL_TOOL})
  else()
    message(FATAL_ERROR "Environment variable RVBL_TOOL not defined, please \
source \"bootstrap.sh\".")
  endif()

  find_program(ASCIIDOCTOR NAMES asciidoctor REQUIRED)

  find_program(
    EXTRACTOR_ASCIIDOC
    NAMES asciidoc_extractor
    HINTS ${rvbl_tool} REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    GENERATOR_ASCIIDOC_MACHINE
    NAMES generator_asciidoc_machine
    HINTS ${rvbl_tool} REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    GENERATOR_ASCIIDOC_PERIPHERAL
    NAMES generator_asciidoc_peripheral
    HINTS ${rvbl_tool} REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    GENERATOR_LINKER
    NAMES generator_linker
    HINTS ${rvbl_tool} REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    GENERATOR_C
    NAMES generator_c
    HINTS ${rvbl_tool} REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    MODEL_PARSER
    NAMES parser
    HINTS ${rvbl_tool} REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    RAVE
    NAMES rave
    HINTS ${rvbl_tool} REQUIRED
    NO_DEFAULT_PATH)

  find_program(TRUNCATE NAMES truncate)

  # Setup rvbl_toolchain_moniker global property
  cmake_path(GET CMAKE_TOOLCHAIN_FILE PARENT_PATH cmake_toolchain_path)
  execute_process(COMMAND ${RAVE} moniker ${cmake_toolchain_path}
                  OUTPUT_VARIABLE rvbl_toolchain_moniker)
  string(STRIP "${rvbl_toolchain_moniker}" rvbl_toolchain_moniker)
  set_property(GLOBAL PROPERTY rvbl_toolchain_moniker ${rvbl_toolchain_moniker})

  # Setup rvbl_machine_moniker global property
  execute_process(COMMAND ${RAVE} moniker ${CMAKE_CURRENT_SOURCE_DIR}
                  OUTPUT_VARIABLE rvbl_machine_moniker)
  string(STRIP "${rvbl_machine_moniker}" rvbl_machine_moniker)
  set_property(GLOBAL PROPERTY rvbl_machine_moniker ${rvbl_machine_moniker})

  # Setup rvbl_runners global property
  if(arg_RUNNERS)
    set_property(GLOBAL PROPERTY rvbl_runners ${arg_RUNNERS})
  endif()

  # Setup rvbl_rave_local global property
  string(REGEX REPLACE "docker/[^/]+/" "local/" rvbl_rave_local ${RAVE})
  set_property(GLOBAL PROPERTY rvbl_rave_local ${rvbl_rave_local})

  # Setup rvbl_toolchain_dir global property
  cmake_path(GET CMAKE_TOOLCHAIN_FILE PARENT_PATH rvbl_toolchain_dir)
  set_property(GLOBAL PROPERTY rvbl_toolchain_dir ${rvbl_toolchain_dir})

  # Setup various common global properties
  set_property(GLOBAL PROPERTY rvbl_root ${rvbl_root})
  set_property(GLOBAL PROPERTY rvbl_tool ${rvbl_tool})
  set(rvbl_machine_dir ${CMAKE_CURRENT_SOURCE_DIR})
  set_property(GLOBAL PROPERTY rvbl_machine_dir ${rvbl_machine_dir})

  if(arg_BOOTLOADER)
    set_property(GLOBAL PROPERTY rvbl_machine_bootloader ${arg_BOOTLOADER})
  endif()

  # Run generators
  set(model_path ${rvbl_machine_dir}/model/model.json)
  set(generated_path ${rvbl_machine_dir}/generated)
  set(linker_path ${generated_path}/linker)
  set(documentation_path ${generated_path}/doc)

  file(MAKE_DIRECTORY ${generated_path} ${documentation_path} ${linker_path})

  rvbl_compute_model_dependencies(MODEL ${model_path} OUTPUT_VARIABLE
                                  model_files)
  rvbl_generate_c_files(
    MODEL
    ${model_path}
    PATH
    ${generated_path}
    DEPENDENCIES
    ${model_files}
    OUTPUT_VARIABLE
    generated_files_c)
  rvbl_generate_linker_files(
    MODEL
    ${model_path}
    PATH
    ${linker_path}
    DEPENDENCIES
    ${model_files}
    OUTPUT_VARIABLE
    generated_files_linker)
  rvbl_generate_machine_documentation(
    MODEL ${model_path} PATH ${documentation_path} DEPENDENCIES
    ${generated_files_c})

  # Configure machine library
  add_library(
    rvbl_machine STATIC ${generated_files_c}
                        ${rvbl_machine_dir}/source/rvbl_configuration.c)
  target_include_directories(rvbl_machine
                             PUBLIC ${rvbl_machine_dir}/generated/include)
  target_link_libraries(
    rvbl_machine PUBLIC rvbl_type rvbl_hardware rvbl_boot
                        $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)
  add_dependencies(rvbl_machine machine_c_files machine_linker_files
                   machine_doc_files)

  install(TARGETS rvbl_machine)
  install(FILES ${rvbl_machine_dir}/linker/${rvbl_linker_id}.ld
          DESTINATION linker/machine)
endfunction()

function(rvbl_compute_model_dependencies)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "MODEL;OUTPUT_VARIABLE" "")
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  execute_process(
    COMMAND ${MODEL_PARSER} --input ${arg_MODEL} --meta-model
            ${rvbl_root}/core/meta --search-path ${rvbl_root} --dry-run
    OUTPUT_VARIABLE model_files # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${model_files}" model_files)
  list(TRANSFORM model_files PREPEND "${rvbl_root}/")
  list(APPEND model_files ${arg_MODEL})
  set_property(
    DIRECTORY
    APPEND
    PROPERTY CMAKE_CONFIGURE_DEPENDS "${model_files}")

  set(${arg_OUTPUT_VARIABLE} ${model_files})

  return(PROPAGATE ${arg_OUTPUT_VARIABLE})
endfunction()

function(rvbl_generate_c_files)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "MODEL;PATH;OUTPUT_VARIABLE"
                        "DEPENDENCIES")
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  execute_process(
    COMMAND ${GENERATOR_C} --input ${arg_MODEL} --meta-model
            ${rvbl_root}/core/meta --search-path ${rvbl_root} --dry-run
    WORKING_DIRECTORY ${arg_PATH}
    OUTPUT_VARIABLE generated_files
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${generated_files}" generated_files)
  list(TRANSFORM generated_files PREPEND ${arg_PATH}/)

  add_custom_command(
    OUTPUT ${generated_files}
    COMMAND ${GENERATOR_C} --input ${arg_MODEL} --meta-model
            ${rvbl_root}/core/meta --search-path ${rvbl_root}
    DEPENDS ${arg_DEPENDENCIES}
    WORKING_DIRECTORY ${arg_PATH}
    COMMENT "Generating machine headers from ${arg_MODEL} ...")

  add_custom_target(machine_c_files DEPENDS ${generated_files})

  set(${arg_OUTPUT_VARIABLE} ${generated_files})

  install(FILES ${generated_files} DESTINATION include/machine)

  return(PROPAGATE ${arg_OUTPUT_VARIABLE})
endfunction()

function(rvbl_generate_linker_files)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "MODEL;PATH;OUTPUT_VARIABLE"
                        "DEPENDENCIES")
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  execute_process(
    COMMAND ${GENERATOR_LINKER} --input ${arg_MODEL} --meta-model
            ${rvbl_root}/core/meta --search-path ${rvbl_root} --dry-run
    WORKING_DIRECTORY ${arg_PATH}
    OUTPUT_VARIABLE generated_files
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${generated_files}" generated_files)
  list(TRANSFORM generated_files PREPEND "${arg_PATH}/")

  add_custom_command(
    OUTPUT ${generated_files}
    COMMAND ${GENERATOR_LINKER} --input ${arg_MODEL} --meta-model
            ${rvbl_root}/core/meta --search-path ${rvbl_root}
    DEPENDS ${arg_DEPENDENCIES}
    WORKING_DIRECTORY ${arg_PATH}
    COMMENT "Generating linker files from ${arg_MODEL} ..."
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )

  add_custom_target(machine_linker_files DEPENDS ${generated_files})

  set(${arg_OUTPUT_VARIABLE} ${generated_files})

  install(FILES ${generated_files} DESTINATION linker/machine)

  return(PROPAGATE ${arg_OUTPUT_VARIABLE})
endfunction()

function(rvbl_generate_machine_documentation)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "MODEL;PATH" "DEPENDENCIES")
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  # Model documentation
  add_custom_command(
    OUTPUT ${arg_PATH}/model.adoc ${arg_PATH}/model.html
    COMMAND ${GENERATOR_ASCIIDOC_MACHINE} --input ${arg_MODEL} --meta-model
            ${rvbl_root}/core/meta --search-path ${rvbl_root}
    COMMAND ${ASCIIDOCTOR} -o ${arg_PATH}/model.html ${arg_PATH}/model.adoc
    WORKING_DIRECTORY ${arg_PATH}
    DEPENDS ${arg_DEPENDENCIES}
    COMMENT "Generating model documentation from ${arg_MODEL} ...")
  add_custom_target(model_doc ALL DEPENDS ${arg_PATH}/model.html)

  # Machine library documentation
  set(adoc_list "")
  set(html_list "")
  set(counter "0")

  foreach(source ${arg_DEPENDENCIES})
    get_filename_component(source_name_we ${source} NAME_WE)
    set(adoc_output ${arg_PATH}/${source_name_we}.adoc)

    if(${adoc_output} IN_LIST adoc_list)
      string(APPEND source_name_we _${counter})
      set(adoc_output ${arg_PATH}/${source_name_we}.adoc)
      math(EXPR counter "${counter} + 1")
    endif()

    add_custom_command(
      OUTPUT ${adoc_output}
      COMMAND ${EXTRACTOR_ASCIIDOC} ${source} ${adoc_output}
      DEPENDS ${source}
      COMMENT "Generating AsciiDoc documentation from ${source} ...")

    list(APPEND adoc_list ${adoc_output})
  endforeach()

  foreach(source ${adoc_list})
    get_filename_component(source_name_we ${source} NAME_WE)
    set(html_output ${arg_PATH}/${source_name_we}.html)

    add_custom_command(
      OUTPUT ${html_output}
      COMMAND ${ASCIIDOCTOR} ${source} -o ${html_output}
      DEPENDS ${adoc_list}
      COMMENT "Generating HTML documentation from ${source} ...")

    list(APPEND html_list ${html_output})
  endforeach()

  install(FILES ${html_list} DESTINATION doc/lib/machine)

  add_custom_target(machine_doc_files ALL DEPENDS ${html_list})
endfunction()

function(rvbl_library_documentation module)
  set(generated_path "${CMAKE_CURRENT_SOURCE_DIR}/generated")
  set(documentation_path "${CMAKE_CURRENT_SOURCE_DIR}/generated")
  file(MAKE_DIRECTORY ${generated_path} ${documentation_path})

  get_target_property(sources rvbl_${module} SOURCES)
  add_custom_command(
    OUTPUT "${documentation_path}/${module}.html"
    COMMAND "${EXTRACTOR_ASCIIDOC}" ${sources}
            "${documentation_path}/${module}.adoc"
    COMMAND ${ASCIIDOCTOR} -o "${documentation_path}/${module}.html"
            "${documentation_path}/${module}.adoc"
    DEPENDS ${sources}
    BYPRODUCTS "${documentation_path}/${module}.adoc"
    COMMENT "Generating documentation for ${module} ...")
  add_custom_target(${module}_doc ALL
                    DEPENDS "${documentation_path}/${module}.html")
  install(FILES ${documentation_path}/${module}.html
          DESTINATION doc/lib/${module})
endfunction()

function(rvbl_interface_library)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "NAME" "SOURCES;INCLUDES;LIBRARIES")

  add_library(rvbl_${arg_NAME} INTERFACE ${arg_SOURCES})
  target_include_directories(rvbl_${arg_NAME} INTERFACE ${arg_INCLUDES})
  target_link_libraries(
    rvbl_${arg_NAME} INTERFACE $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)

  rvbl_library_documentation(${arg_NAME})
  get_target_property(sources rvbl_${arg_NAME} SOURCES)
  set(headers "")

  foreach(item ${sources})
    if(${item} MATCHES ".h$")
      list(APPEND headers ${item})
    endif()
  endforeach()

  install(FILES ${headers} DESTINATION include/${arg_NAME})
endfunction()

function(rvbl_static_library)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "NAME"
                        "SOURCES;INCLUDES;LIBRARIES;DEFINITIONS")

  add_library(rvbl_${arg_NAME} STATIC ${arg_SOURCES})
  target_include_directories(rvbl_${arg_NAME} PUBLIC ${arg_INCLUDES})
  target_link_libraries(rvbl_${arg_NAME}
                        PUBLIC $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)
  target_compile_definitions(rvbl_${arg_NAME} PRIVATE ${arg_DEFINITIONS})

  rvbl_library_documentation(${arg_NAME})

  install(TARGETS rvbl_${arg_NAME})
  get_target_property(sources rvbl_${arg_NAME} SOURCES)
  set(headers "")

  foreach(item ${sources})
    if(${item} MATCHES ".h$")
      list(APPEND headers ${item})
    endif()
  endforeach()

  install(FILES ${headers} DESTINATION include/${arg_NAME})
endfunction()

function(rvbl_executable)
  cmake_parse_arguments(
    PARSE_ARGV 0 arg "TEST;BINARY;FLASH"
    "NAME;TEST_TIMEOUT;TEST_EXPECTED_OUTPUT;BINARY_SIZE"
    "SOURCES;LIBRARIES;LIBRARIES_EXTERNAL;DEFINITIONS;INCLUDES")

  get_property(rvbl_machine_dir GLOBAL PROPERTY rvbl_machine_dir)
  get_property(rvbl_toolchain_dir GLOBAL PROPERTY rvbl_toolchain_dir)
  get_property(rvbl_rave_local GLOBAL PROPERTY rvbl_rave_local)
  get_property(rvbl_toolchain_moniker GLOBAL PROPERTY rvbl_toolchain_moniker)
  get_property(rvbl_machine_moniker GLOBAL PROPERTY rvbl_machine_moniker)
  get_property(rvbl_runners GLOBAL PROPERTY rvbl_runners)

  add_executable(${arg_NAME} ${arg_SOURCES})
  target_link_options(${arg_NAME} PRIVATE ${rvbl_linker_nostdlib})
  target_link_libraries(
    ${arg_NAME} PRIVATE ${arg_LIBRARIES_EXTERNAL}
                        $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)
  target_compile_definitions(${arg_NAME} PRIVATE ${arg_DEFINITIONS})
  target_include_directories(${arg_NAME} PRIVATE ${arg_INCLUDES})
  set_target_properties(
    ${arg_NAME}
    PROPERTIES LINKER_SCRIPT "${rvbl_machine_dir}/linker/\
${rvbl_linker_id}.ld" LINKER_SEARCH_DIRECTORIES "${rvbl_toolchain_dir};\
${rvbl_machine_dir}/generated/linker")
  add_dependencies(${arg_NAME} machine_c_files machine_linker_files)

  install(TARGETS ${arg_NAME})

  if(arg_BINARY)
    add_custom_command(
      TARGET ${arg_NAME}
      POST_BUILD
      COMMAND ${OBJCOPY} $<TARGET_FILE:${arg_NAME}> ${rvbl_elf_copy_parameters}
              ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}.bin
      BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}.bin
      COMMENT "Converting $<TARGET_FILE:${arg_NAME}> to binary image...")

    if(arg_BINARY_SIZE)
      if(TRUNCATE)
        add_custom_command(
          TARGET ${arg_NAME}
          POST_BUILD
          COMMAND ${TRUNCATE} -s ${arg_BINARY_SIZE}
                  ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}.bin
          COMMENT "Resizing ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}.bin ...")
      else()
        message(WARNING "No \"truncate\" executable found, ${arg_NAME}.bin \
shall be manually resized to ${arg_BINARY_SIZE}.")
      endif()
    endif()

    install(FILES ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}.bin DESTINATION bin)
  endif()

  if(arg_TEST)
    if("qemu" IN_LIST rvbl_runners OR "qemu-docker" IN_LIST rvbl_runners)
      set(test_list "")
      set(extra "")

      if("qemu-docker" IN_LIST rvbl_runners)
        set(extra "--docker")
      endif()

      add_test(
        NAME qemu.${arg_NAME}
        COMMAND
          ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker}
          --machine ${rvbl_machine_moniker} --runner qemu.system32-elf --file
          ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C}
          ${extra})
      list(APPEND test_list qemu.${arg_NAME})

      if(arg_FLASH)
        add_test(
          NAME qemu.${arg_NAME}.flash
          COMMAND
            ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker}
            --machine ${rvbl_machine_moniker} --runner qemu.system32-bin --file
            ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}_flash.bin ${extra})
        list(APPEND test_list qemu.${arg_NAME}.flash)
      endif()

      if(arg_TEST_EXPECTED_OUTPUT)
        set_tests_properties(
          ${test_list} PROPERTIES PASS_REGULAR_EXPRESSION
                                  ${arg_TEST_EXPECTED_OUTPUT})
      endif()

      if(arg_TEST_TIMEOUT)
        set_tests_properties(${test_list} PROPERTIES TIMEOUT
                                                     ${arg_TEST_TIMEOUT})
      endif()

      set_tests_properties(qemu.${arg_NAME} PROPERTIES SKIP_RETURN_CODE 2)
    endif()

    if("jrun" IN_LIST rvbl_runners)
      set(test_list "")

      add_test(
        NAME jrun.${arg_NAME}
        COMMAND
          ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker}
          --machine ${rvbl_machine_moniker} --runner segger.jrun --file
          ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C})
      list(APPEND test_list jrun.${arg_NAME})

      if(arg_TEST_EXPECTED_OUTPUT)
        set_tests_properties(
          ${test_list} PROPERTIES PASS_REGULAR_EXPRESSION
                                  ${arg_TEST_EXPECTED_OUTPUT})
      endif()

      if(arg_TEST_TIMEOUT)
        set_tests_properties(${test_list} PROPERTIES TIMEOUT
                                                     ${arg_TEST_TIMEOUT})
      endif()

      set_tests_properties(jrun.${arg_NAME} PROPERTIES SKIP_RETURN_CODE 2)
    endif()

    if("vdk" IN_LIST rvbl_runners)
      set(test_list "")

      add_test(
        NAME vdk.${arg_NAME}
        COMMAND
          ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker}
          --machine ${rvbl_machine_moniker} --runner
          quintauris_internal.vdk-elf --file
          ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C})
      list(APPEND test_list vdk.${arg_NAME})

      if(arg_TEST_EXPECTED_OUTPUT)
        message(WARNING "Ignoring TEST_EXPECTED_OUTPUT parameter in VDK tests.")
      endif()

      if(arg_TEST_TIMEOUT)
        message(WARNING "Ignoring TEST_TIMEOUT parameter in VDK tests.")
      endif()

      set_tests_properties(
        ${test_list}
        PROPERTIES PASS_REGULAR_EXPRESSION
                   "PASS"
                   FAIL_REGULAR_EXPRESSION
                   "FAIL"
                   SKIP_REGULAR_EXPRESSION
                   "SKIP"
                   TIMEOUT
                   120)
    endif()

    if("t32" IN_LIST rvbl_runners)
      set(test_list "")

      add_test(
        NAME t32.${arg_NAME}
        COMMAND
          ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker}
          --machine ${rvbl_machine_moniker} --runner lauterbach.t32 --delay 8
          --dump-file output.txt --file
          ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C})
      list(APPEND test_list vdk.${arg_NAME})

      if(arg_TEST_EXPECTED_OUTPUT)
        message(WARNING "Ignoring TEST_EXPECTED_OUTPUT parameter in T32 tests.")
      endif()

      if(arg_TEST_TIMEOUT)
        message(WARNING "Ignoring TEST_TIMEOUT parameter in T32 tests.")
      endif()

      set_tests_properties(
        t32.${arg_NAME}
        PROPERTIES PASS_REGULAR_EXPRESSION "PASS" FAIL_REGULAR_EXPRESSION
                   "FAIL" SKIP_REGULAR_EXPRESSION "SKIP")
    endif()

    if("openocd" IN_LIST rvbl_runners)
      set(test_list "")

      add_test(
        NAME openocd.${arg_NAME}
        COMMAND
          ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker}
          --machine ${rvbl_machine_moniker} --runner quintauris.openocd --file
          ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C})
      list(APPEND test_list openocd.${arg_NAME})

      if(arg_TEST_EXPECTED_OUTPUT)
        set_tests_properties(
          ${test_list} PROPERTIES PASS_REGULAR_EXPRESSION
                                  ${arg_TEST_EXPECTED_OUTPUT})
      else()
        set_tests_properties(
          ${test_list}
          PROPERTIES PASS_REGULAR_EXPRESSION "PASS" FAIL_REGULAR_EXPRESSION
                     "FAIL" SKIP_REGULAR_EXPRESSION "SKIP")
      endif()

      if(arg_TEST_TIMEOUT)
        set_tests_properties(${test_list} PROPERTIES TIMEOUT
                                                     ${arg_TEST_TIMEOUT})
      endif()
    endif()
  endif()

  if(arg_FLASH)
    set(flash_executable_name "${arg_NAME}_flash")

    add_executable(${flash_executable_name} ${arg_SOURCES})
    target_link_options(${flash_executable_name} PRIVATE
                        ${rvbl_linker_nostdlib})
    target_link_libraries(
      ${flash_executable_name}
      PRIVATE $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>
              ${arg_LIBRARIES_EXTERNAL})
    target_compile_definitions(${flash_executable_name}
                               PRIVATE RVBL_BUILD_FLASH ${arg_DEFINITIONS})
    target_include_directories(${flash_executable_name} PRIVATE ${arg_INCLUDES})
    set_target_properties(
      ${flash_executable_name}
      PROPERTIES LINKER_SCRIPT "${rvbl_machine_dir}/linker/\
${rvbl_linker_id}_flash.ld" LINKER_SEARCH_DIRECTORIES "${rvbl_toolchain_dir};\
${rvbl_machine_dir}/generated/linker")

    if(arg_BINARY)
      add_custom_target(
        ${flash_executable_name}_binary ALL
        ${OBJCOPY} $<TARGET_FILE:${flash_executable_name}>
        ${rvbl_elf_copy_parameters}
        ${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}.bin
        BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}.bin
        COMMENT
          "Converting $<TARGET_FILE:${flash_executable_name}> to binary image \
...")

      get_property(rvbl_machine_bootloader GLOBAL
                   PROPERTY rvbl_machine_bootloader)

      if(rvbl_machine_bootloader)
        add_custom_command(
          TARGET ${flash_executable_name}_binary
          POST_BUILD
          COMMAND
            ${CMAKE_COMMAND} -E cat ${rvbl_machine_bootloader}
            ${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}.bin >
            ${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}_bootable.bin
          BYPRODUCTS
            ${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}_bootable.bin
          COMMENT "Prepending bootloader to \
${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}.bin ...")
      endif()

      if(arg_BINARY_SIZE)
        if(TRUNCATE)
          add_custom_command(
            TARGET ${flash_executable_name}_binary
            POST_BUILD
            COMMAND ${TRUNCATE} -s ${arg_BINARY_SIZE}
                    ${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}.bin
            COMMENT "Resizing \
${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}.bin ...")
        else()
          message(
            WARNING
              "No \"truncate\" executable found, ${flash_executable_name}.bin \
  shall be manually resized to ${arg_BINARY_SIZE}.")
        endif()
      endif()
    endif()
  endif()
endfunction()

function(rvbl_test)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "NAME;TIMEOUT;EXPECTED_OUTPUT"
                        "LIBRARIES;DEFINITIONS")

  if(DEFINED arg_EXPECTED_OUTPUT)
    rvbl_executable(
      NAME
      test_${arg_NAME}
      SOURCES
      ${CMAKE_CURRENT_SOURCE_DIR}/tests/${arg_NAME}.c
      LIBRARIES
      test
      ${arg_LIBRARIES}
      DEFINITIONS
      ${arg_DEFINITIONS}
      TEST
      TEST_EXPECTED_OUTPUT
      ${arg_EXPECTED_OUTPUT})
  else()
    rvbl_executable(
      NAME
      test_${arg_NAME}
      SOURCES
      ${CMAKE_CURRENT_SOURCE_DIR}/tests/${arg_NAME}.c
      LIBRARIES
      test
      ${arg_LIBRARIES}
      DEFINITIONS
      ${arg_DEFINITIONS}
      TEST)
  endif()

endfunction()

macro(rvbl_use)
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  foreach(parameter ${ARGV})
    add_subdirectory(${rvbl_root}/${parameter}
                     ${CMAKE_CURRENT_BINARY_DIR}/${parameter})
  endforeach()
endmacro()

function(rvbl_layer_documentation_model)
  cmake_parse_arguments(
    PARSE_ARGV 0 arg "" "SOURCE_DIR;TARGET_DIR;RVBL_ROOT;GENERATOR"
    "MODELS;PARAMETERS")

  find_program(GENERATOR_ASCIIDOC_${arg_GENERATOR}
               NAMES generator_asciidoc_${arg_GENERATOR} REQUIRED)

  set(temp_dir ${arg_SOURCE_DIR}/generated)
  set(outputs_model_${arg_GENERATOR} "")

  foreach(input ${arg_MODELS})
    get_filename_component(file_name ${input} NAME_WE)
    get_filename_component(model_name ${input} DIRECTORY)
    get_filename_component(model_name ${model_name} NAME)
    set(output_adoc ${temp_dir}/${model_name}_${file_name}.adoc)
    list(APPEND outputs_model_${arg_GENERATOR} ${output_adoc})

    add_custom_command(
      OUTPUT ${output_adoc}
      COMMAND
        ${GENERATOR_ASCIIDOC_${arg_GENERATOR}} --input ${input} --meta-model
        ${arg_RVBL_ROOT}/core/meta --search-path ${arg_RVBL_ROOT}
        $<$<BOOL:${arg_PARAMETERS}>:--parameter> ${arg_PARAMETERS}
      COMMAND ${CMAKE_COMMAND} -E rename ${temp_dir}/model.adoc ${output_adoc}
      DEPENDS ${input}
      WORKING_DIRECTORY ${temp_dir}
      COMMENT "Generating model ${model_name}/${file_name} documentation...")
  endforeach()

  return(PROPAGATE outputs_model_${arg_GENERATOR})
endfunction()

function(rvbl_layer_documentation)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "SOURCE_DIR;TARGET_DIR;RVBL_ROOT"
                        "PERIPHERAL_MODELS;CPU_MODELS;PARAMETERS")

  make_directory(${arg_TARGET_DIR})
  set(target_dir_adoc ${arg_SOURCE_DIR}/generated)
  set(target_dir_html ${arg_TARGET_DIR})
  cmake_path(GET arg_SOURCE_DIR PARENT_PATH layer_dir)
  cmake_path(GET layer_dir STEM layer_name)
  make_directory(${target_dir_adoc})
  make_directory(${target_dir_html})

  find_program(ASCIIDOCTOR NAMES asciidoctor REQUIRED)
  find_program(EXTRACTOR_ASCIIDOC NAMES asciidoc_extractor REQUIRED)

  # Libraries
  file(
    GLOB_RECURSE inputs
    LIST_DIRECTORIES false
    RELATIVE ${arg_SOURCE_DIR}
    "${arg_SOURCE_DIR}/../lib/*.h")

  if(inputs)
    list(SORT inputs)
    set(outputs_api ${target_dir_adoc}/api.adoc)
    add_custom_command(
      OUTPUT ${outputs_api}
      COMMAND ${EXTRACTOR_ASCIIDOC} ${inputs} ${outputs_api}
      COMMAND_EXPAND_LISTS
      DEPENDS ${inputs}
      WORKING_DIRECTORY ${arg_SOURCE_DIR}
      COMMENT "Generating API documentation...")
  endif()

  # Models
  rvbl_layer_documentation_model(
    SOURCE_DIR
    ${arg_SOURCE_DIR}
    TARGET_DIR
    ${target_dir_adoc}
    RVBL_ROOT
    ${arg_RVBL_ROOT}
    GENERATOR
    peripheral
    MODELS
    ${arg_PERIPHERAL_MODELS}
    PARAMETERS
    ${arg_PARAMETERS})

  # Main documents and common files
  file(GLOB inputs "${arg_SOURCE_DIR}/*.adoc")
  list(APPEND inputs ${outputs_model_peripheral})
  set(outputs_doc "")

  foreach(input ${inputs})
    get_filename_component(file_name_we ${input} NAME_WE)
    list(APPEND outputs_doc ${target_dir_html}/${file_name_we}.html)
  endforeach()

  add_custom_command(
    OUTPUT ${outputs_doc}
    COMMAND ${ASCIIDOCTOR} --doctype book -D ${target_dir_html} ${inputs}
    DEPENDS ${inputs} ${outputs_api} ${outputs_model_peripheral}
    COMMENT "Generating documentation...")

  file(GLOB glob "${arg_RVBL_ROOT}/doc/*.png")
  file(COPY ${glob} DESTINATION ${target_dir_html})

  add_custom_target(
    ${layer_name}_docs ALL DEPENDS ${outputs_api} ${outputs_model_peripheral}
                                   ${outputs_doc})

  install(DIRECTORY ${target_dir_html} DESTINATION doc/layer/${layer_name})
endfunction()
