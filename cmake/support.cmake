function(rvbl_machine)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "" "LIBRARIES;RUNNERS")

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
    GENERATOR_ASCIIDOC
    NAMES generator_asciidoc
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
    rvbl_machine STATIC
    ${generated_files_c} ${rvbl_machine_dir}/source/rvbl_configuration.c
    ${rvbl_machine_dir}/source/rvbl_test.c)
  target_include_directories(rvbl_machine
                             PUBLIC ${rvbl_machine_dir}/generated/include)
  target_link_libraries(
    rvbl_machine PUBLIC rvbl_core rvbl_boot
                        $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)
endfunction()

function(rvbl_compute_model_dependencies)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "MODEL;OUTPUT_VARIABLE" "")
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  execute_process(
    COMMAND
      ${MODEL_PARSER} ${arg_MODEL}
      ${rvbl_root}/quintauris/meta/machine.schema.json ${rvbl_root}
      --dependencies
    OUTPUT_VARIABLE model_files
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
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
    COMMAND ${GENERATOR_C} ${arg_MODEL}
            ${rvbl_root}/quintauris/meta/machine.schema.json ${rvbl_root} --dry
    WORKING_DIRECTORY ${arg_PATH}
    OUTPUT_VARIABLE generated_files
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${generated_files}" generated_files)
  list(TRANSFORM generated_files PREPEND ${arg_PATH}/)

  add_custom_command(
    OUTPUT ${generated_files}
    COMMAND ${GENERATOR_C} ${arg_MODEL}
            ${rvbl_root}/quintauris/meta/machine.schema.json ${rvbl_root}
    DEPENDS ${arg_DEPENDENCIES}
    WORKING_DIRECTORY ${arg_PATH}
    COMMENT "Generating machine headers from ${arg_MODEL} ...")

  message("${generated_files}")

  add_custom_target(machine_c_files ALL DEPENDS ${generated_files})

  set(${arg_OUTPUT_VARIABLE} ${generated_files})

  return(PROPAGATE ${arg_OUTPUT_VARIABLE})
endfunction()

function(rvbl_generate_linker_files)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "MODEL;PATH;OUTPUT_VARIABLE"
                        "DEPENDENCIES")
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  execute_process(
    COMMAND ${GENERATOR_LINKER} ${arg_MODEL}
            ${rvbl_root}/quintauris/meta/machine.schema.json ${rvbl_root} --dry
    WORKING_DIRECTORY ${arg_PATH}
    OUTPUT_VARIABLE generated_files
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${generated_files}" generated_files)
  list(TRANSFORM generated_files PREPEND "${arg_PATH}/")

  add_custom_command(
    OUTPUT ${generated_files}
    COMMAND ${GENERATOR_LINKER} ${arg_MODEL}
            ${rvbl_root}/quintauris/meta/machine.schema.json ${rvbl_root}
    DEPENDS ${arg_DEPENDENCIES}
    WORKING_DIRECTORY ${arg_PATH}
    COMMENT "Generating linker files from ${arg_MODEL} ..."
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )

  add_custom_target(machine_linker_files ALL DEPENDS ${generated_files})

  set(${arg_OUTPUT_VARIABLE} ${generated_files})

  return(PROPAGATE ${arg_OUTPUT_VARIABLE})
endfunction()

function(rvbl_generate_machine_documentation)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "MODEL;PATH" "DEPENDENCIES")
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  # Model documentation
  add_custom_command(
    OUTPUT ${arg_PATH}/model.adoc ${arg_PATH}/model.html
    COMMAND ${GENERATOR_ASCIIDOC} ${arg_MODEL}
            ${rvbl_root}/quintauris/meta/machine.schema.json ${rvbl_root}
    COMMAND ${ASCIIDOCTOR} -o ${arg_PATH}/model.html ${arg_PATH}/model.adoc
    WORKING_DIRECTORY ${arg_PATH}
    DEPENDS ${arg_DEPENDENCIES}
    COMMENT "Generating model documentation from ${arg_MODEL} ...")
  add_custom_target(model_doc ALL DEPENDS ${arg_PATH}/model.html)

  # Machine library documentation
  set(machine_list "")
  set(counter "0")

  foreach(source ${arg_DEPENDENCIES})
    get_filename_component(source_name_we ${source} NAME_WE)
    set(adoc_output ${arg_PATH}/${source_name_we}.adoc)

    if(${adoc_output} IN_LIST machine_list)
      string(APPEND source_name_we _${counter})
      set(adoc_output ${arg_PATH}/${source_name_we}.adoc)
      math(EXPR counter "${counter} + 1")
    endif()

    add_custom_command(
      OUTPUT ${adoc_output}
      COMMAND ${EXTRACTOR_ASCIIDOC} ${source} ${adoc_output}
      DEPENDS ${source})
    list(APPEND machine_list ${adoc_output})
  endforeach()

  add_custom_command(
    OUTPUT ${arg_PATH}/rvbl/machine/rvbl_machine.html
    COMMAND ${ASCIIDOCTOR} -o {arg_PATH}/rvbl/machine/rvbl_machine.html
            ${arg_PATH}/rvbl_machine.adoc
    DEPENDS ${machine_list}
    COMMENT "Generating source documentation from ${arg_MODEL} ...")
  add_custom_target(machine_doc_files ALL
                    DEPENDS ${arg_PATH}/rvbl/machine/rvbl_machine.html)
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

  rvbl_library_documentation(${arg_NAME})
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

  rvbl_library_documentation(${arg_NAME})
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
  get_property(rvbl_toolchain_dir GLOBAL PROPERTY rvbl_toolchain_dir)
  get_property(rvbl_machine_dir GLOBAL PROPERTY rvbl_machine_dir)

  add_executable(test_${arg_NAME}
                 "${init}" "${CMAKE_CURRENT_SOURCE_DIR}/tests/${arg_NAME}.c")
  target_link_options(test_${arg_NAME} PRIVATE ${rvbl_linker_nostdlib})
  target_link_libraries(
    test_${arg_NAME} PRIVATE rvbl_test
                             $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)
  set_target_properties(
    test_${arg_NAME}
    PROPERTIES LINKER_SCRIPT "${rvbl_machine_dir}/linker/\
${rvbl_linker_id}.ld" LINKER_SEARCH_DIRECTORIES "${rvbl_toolchain_dir};\
${rvbl_machine_dir}/generated/linker")

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

macro(rvbl_use)
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  foreach(parameter ${ARGV})
    add_subdirectory(${rvbl_root}/${parameter}
                     ${CMAKE_CURRENT_BINARY_DIR}/${parameter})
  endforeach()
endmacro()
