function(rvbl_machine)
  set(options "")
  set(oneValueArgs "")
  set(multiValueArgs RUNNERS)
  cmake_parse_arguments(PARSE_ARGV 0 arg "${options}" "${oneValueArgs}"
                        "${multiValueArgs}")

  find_program(ASCIIDOCTOR NAMES asciidoctor REQUIRED)

  find_program(
    EXTRACTOR_ASCIIDOC
    NAMES asciidoc_extractor
    HINTS ${host_tools} ${host_tools}/release ${host_tools}/debug REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    GENERATOR_ASCIIDOC
    NAMES generator_asciidoc
    HINTS ${host_tools} ${host_tools}/release ${host_tools}/debug REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    GENERATOR_LINKER
    NAMES generator_linker
    HINTS ${host_tools} ${host_tools}/release ${host_tools}/debug REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    GENERATOR_C
    NAMES generator_c
    HINTS ${host_tools} ${host_tools}/release ${host_tools}/debug REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    MODEL_PARSER
    NAMES parser
    HINTS ${host_tools} ${host_tools}/release ${host_tools}/debug REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    RAVE
    NAMES rave
    HINTS ${host_tools} ${host_tools}/release ${host_tools}/debug REQUIRED
    NO_DEFAULT_PATH)

  # Setup rvbl_toolchain_moniker global property
  cmake_path(GET CMAKE_TOOLCHAIN_FILE PARENT_PATH cmake_toolchain_path)
  execute_process(COMMAND ${RAVE} moniker ${cmake_toolchain_path}
                  OUTPUT_VARIABLE rvbl_toolchain_moniker)
  string(STRIP "${rvbl_toolchain_moniker}" rvbl_toolchain_moniker)
  set_property(GLOBAL PROPERTY rvbl_toolchain_moniker ${rvbl_toolchain_moniker})

  # Setup rvbl_machine_moniker global property
  execute_process(COMMAND ${RAVE} moniker ${CMAKE_SOURCE_DIR}
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
endfunction()

function(rvbl_run_generators)
  set(vendor_path "${CMAKE_SOURCE_DIR}/../../..")
  set(generated_path "${CMAKE_CURRENT_SOURCE_DIR}/generated")
  set(documentation_path "${CMAKE_CURRENT_SOURCE_DIR}/generated/doc")
  set(linker_path "${CMAKE_CURRENT_SOURCE_DIR}/generated/linker")
  file(MAKE_DIRECTORY ${generated_path} ${documentation_path} ${linker_path})

  # Compute model dependencies
  execute_process(
    COMMAND
      "${MODEL_PARSER}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json"
      "${vendor_path}/quintauris/meta/machine.schema.json" "${vendor_path}"
      --dependencies
    OUTPUT_VARIABLE model_files
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${model_files}" model_files)
  list(TRANSFORM model_files PREPEND "${vendor_path}/")
  list(APPEND model_files "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json")
  set_property(
    DIRECTORY
    APPEND
    PROPERTY CMAKE_CONFIGURE_DEPENDS "${model_files}")

  # Compute C generator outputs
  execute_process(
    COMMAND
      "${GENERATOR_C}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json"
      "${vendor_path}/quintauris/meta/machine.schema.json" "${vendor_path}"
      --dry
    WORKING_DIRECTORY "${generated_path}"
    OUTPUT_VARIABLE generated_files
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${generated_files}" generated_files)
  list(TRANSFORM generated_files PREPEND "${generated_path}/")

  # Generate C files
  add_custom_command(
    OUTPUT ${generated_files}
    COMMAND
      "${GENERATOR_C}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json"
      "${vendor_path}/quintauris/meta/machine.schema.json" "${vendor_path}"
    DEPENDS ${model_files}
    WORKING_DIRECTORY ${generated_path}
    COMMENT "Generating machine headers from ${CMAKE_CURRENT_SOURCE_DIR}/model/\
model.json ..."
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )

  # Compute Linker generator outputs
  execute_process(
    COMMAND
      "${GENERATOR_LINKER}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json"
      "${vendor_path}/quintauris/meta/machine.schema.json" "${vendor_path}"
      --dry
    WORKING_DIRECTORY "${generated_path}/linker"
    OUTPUT_VARIABLE generated_files_linker
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${generated_files_linker}" generated_files_linker)
  list(TRANSFORM generated_files_linker PREPEND "${generated_path}/")

  # Generate Linker files
  add_custom_command(
    OUTPUT ${generated_files_linker}
    COMMAND
      "${GENERATOR_LINKER}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json"
      "${vendor_path}/quintauris/meta/machine.schema.json" "${vendor_path}"
    DEPENDS "${model_files}"
    WORKING_DIRECTORY "${generated_path}/linker"
    COMMENT "Generating linker files from ${CMAKE_CURRENT_SOURCE_DIR}/model/\
model.json ..."
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  add_custom_target(linker_files ALL DEPENDS ${generated_files_linker})

  # Model documentation
  add_custom_command(
    OUTPUT "${documentation_path}/model.adoc" "${documentation_path}/model.html"
    COMMAND
      "${GENERATOR_ASCIIDOC}" "${CMAKE_CURRENT_SOURCE_DIR}/model/model.json"
      "${vendor_path}/quintauris/meta/machine.schema.json" "${vendor_path}"
    COMMAND "${ASCIIDOCTOR}" -o "${documentation_path}/model.html"
            "${documentation_path}/model.adoc"
    WORKING_DIRECTORY "${documentation_path}"
    DEPENDS "${model_files}"
    COMMENT
      "Generating model documentation from ${CMAKE_CURRENT_SOURCE_DIR}/model/\
model.json ...")
  add_custom_target(model_doc ALL DEPENDS "${documentation_path}/model.html")

  # Machine library documentation
  set(rvbl_machine_sources ${generated_files})
  set(machine_list "")
  set(counter "0")

  foreach(source ${rvbl_machine_sources})
    get_filename_component(source_name_we ${source} NAME_WE)
    set(adoc_output "${documentation_path}/${source_name_we}.adoc")

    if(${adoc_output} IN_LIST machine_list)
      string(APPEND source_name_we "_${counter}")
      set(adoc_output "${documentation_path}/${source_name_we}.adoc")
      math(EXPR counter "${counter} + 1")
    endif()

    add_custom_command(
      OUTPUT ${adoc_output}
      COMMAND ${EXTRACTOR_ASCIIDOC} ${source} ${adoc_output}
      DEPENDS ${source})
    list(APPEND machine_list ${adoc_output})
  endforeach()

  add_custom_command(
    OUTPUT "${documentation_path}/rvbl/machine/rvbl_machine.html"
    COMMAND
      "${ASCIIDOCTOR}" -o "${documentation_path}/rvbl/machine/rvbl_machine.html"
      "${documentation_path}/rvbl_machine.adoc"
    DEPENDS ${machine_list}
    COMMENT
      "Generating source documentation from ${CMAKE_CURRENT_SOURCE_DIR}/model/\
model.json ...")
  add_custom_target(
    machine_doc ALL
    DEPENDS "${documentation_path}/rvbl/machine/rvbl_machine.html")

  return(PROPAGATE generated_files)
endfunction()

function(rvbl_documentation module)
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
endfunction()

function(rvbl_interface_library)
  set(options "")
  set(oneValueArgs NAME)
  set(multiValueArgs SOURCES INCLUDES LIBRARIES)
  cmake_parse_arguments(PARSE_ARGV 0 arg "${options}" "${oneValueArgs}"
                        "${multiValueArgs}")

  add_library(rvbl_${arg_NAME} INTERFACE ${arg_SOURCES})
  target_include_directories(rvbl_${arg_NAME} INTERFACE ${arg_INCLUDES})
  target_link_libraries(
    rvbl_${arg_NAME} INTERFACE $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)

  rvbl_documentation(${arg_NAME})
endfunction()

function(rvbl_static_library)
  set(options "")
  set(oneValueArgs NAME)
  set(multiValueArgs SOURCES INCLUDES LIBRARIES)
  cmake_parse_arguments(PARSE_ARGV 0 arg "${options}" "${oneValueArgs}"
                        "${multiValueArgs}")

  add_library(rvbl_${arg_NAME} STATIC ${arg_SOURCES})
  target_include_directories(rvbl_${arg_NAME} PUBLIC ${arg_INCLUDES})
  target_link_libraries(rvbl_${arg_NAME}
                        PUBLIC $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)

  rvbl_documentation(${arg_NAME})
endfunction()

function(rvbl_test)
  set(options "")
  set(oneValueArgs NAME)
  set(multiValueArgs LIBRARIES DEVICES)
  cmake_parse_arguments(PARSE_ARGV 0 arg "${options}" "${oneValueArgs}"
                        "${multiValueArgs}")

  get_property(rvbl_rave_local GLOBAL PROPERTY rvbl_rave_local)
  get_property(rvbl_toolchain_moniker GLOBAL PROPERTY rvbl_toolchain_moniker)
  get_property(rvbl_machine_moniker GLOBAL PROPERTY rvbl_machine_moniker)
  get_property(rvbl_runners GLOBAL PROPERTY rvbl_runners)

  add_executable(test_${arg_NAME}
                 "${init}" "${CMAKE_CURRENT_SOURCE_DIR}/tests/${arg_NAME}.c")
  target_link_options(test_${arg_NAME} PRIVATE ${rvbl_linker_nostdlib})
  target_link_libraries(
    test_${arg_NAME} PRIVATE rvbl_test
                             $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)
  set_target_properties(
    test_${arg_NAME}
    PROPERTIES LINKER_SCRIPT "${CMAKE_SOURCE_DIR}/linker/${rvbl_linker_id}.ld")

  if("qemu" IN_LIST rvbl_runners)
    add_test(
      NAME qemu.${arg_NAME}
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner qemu.system32-elf --file
        ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C}
    )
    set_tests_properties(qemu.${arg_NAME} PROPERTIES SKIP_RETURN_CODE 2)
  endif()

  if("jrun" IN_LIST rvbl_runners)
    add_test(
      NAME jrun.${arg_NAME}
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner segger.jrun --dump-file
        ${CMAKE_CURRENT_BINARY_DIR}/:tt --file
        ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C}
    )
    set_tests_properties(jrun.${arg_NAME} PROPERTIES SKIP_RETURN_CODE 2)
  endif()

  if("vdk" IN_LIST rvbl_runners)
    add_test(
      NAME vdk.${arg_NAME}
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner quintauris_internal.vdk-elf --file
        ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C}
    )
    set_tests_properties(
      vdk.${arg_NAME}
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
    add_test(
      NAME t32.${arg_NAME}
      COMMAND
        ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker} --machine
        ${rvbl_machine_moniker} --runner lauterbach.t32 --delay 8 --dump-file
        output.txt --file
        ${CMAKE_CURRENT_BINARY_DIR}/test_${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C}
    )
    set_tests_properties(
      t32.${arg_NAME}
      PROPERTIES PASS_REGULAR_EXPRESSION "PASS" FAIL_REGULAR_EXPRESSION "FAIL"
                 SKIP_REGULAR_EXPRESSION "SKIP")
  endif()
endfunction()
