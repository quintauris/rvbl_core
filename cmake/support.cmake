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
    GENERATOR_ASCIIDOC_MACHINE
    NAMES generator_asciidoc_machine
    HINTS ${rvbl_tool} REQUIRED
    NO_DEFAULT_PATH)

  find_program(
    GENERATOR_ASCIIDOC_DEVICE
    NAMES generator_asciidoc_device
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
      ${MODEL_PARSER} --input ${arg_MODEL} --meta-model
      ${rvbl_root}/quintauris/meta/machine.schema.json --search-path
      ${rvbl_root} --dry-run
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
    COMMAND
      ${GENERATOR_C} --input ${arg_MODEL} --meta-model
      ${rvbl_root}/quintauris/meta/machine.schema.json --search-path
      ${rvbl_root} --dry-run
    WORKING_DIRECTORY ${arg_PATH}
    OUTPUT_VARIABLE generated_files
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${generated_files}" generated_files)
  list(TRANSFORM generated_files PREPEND ${arg_PATH}/)

  add_custom_command(
    OUTPUT ${generated_files}
    COMMAND
      ${GENERATOR_C} --input ${arg_MODEL} --meta-model
      ${rvbl_root}/quintauris/meta/machine.schema.json --search-path
      ${rvbl_root}
    DEPENDS ${arg_DEPENDENCIES}
    WORKING_DIRECTORY ${arg_PATH}
    COMMENT "Generating machine headers from ${arg_MODEL} ...")

  add_custom_target(machine_c_files DEPENDS ${generated_files})

  set(${arg_OUTPUT_VARIABLE} ${generated_files})

  return(PROPAGATE ${arg_OUTPUT_VARIABLE})
endfunction()

function(rvbl_generate_linker_files)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "MODEL;PATH;OUTPUT_VARIABLE"
                        "DEPENDENCIES")
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  execute_process(
    COMMAND
      ${GENERATOR_LINKER} --input ${arg_MODEL} --meta-model
      ${rvbl_root}/quintauris/meta/machine.schema.json --search-path
      ${rvbl_root} --dry-run
    WORKING_DIRECTORY ${arg_PATH}
    OUTPUT_VARIABLE generated_files
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )
  string(STRIP "${generated_files}" generated_files)
  list(TRANSFORM generated_files PREPEND "${arg_PATH}/")

  add_custom_command(
    OUTPUT ${generated_files}
    COMMAND
      ${GENERATOR_LINKER} --input ${arg_MODEL} --meta-model
      ${rvbl_root}/quintauris/meta/machine.schema.json --search-path
      ${rvbl_root}
    DEPENDS ${arg_DEPENDENCIES}
    WORKING_DIRECTORY ${arg_PATH}
    COMMENT "Generating linker files from ${arg_MODEL} ..."
    # COMMAND_ECHO STDOUT ECHO_OUTPUT_VARIABLE
  )

  add_custom_target(machine_linker_files DEPENDS ${generated_files})

  set(${arg_OUTPUT_VARIABLE} ${generated_files})

  return(PROPAGATE ${arg_OUTPUT_VARIABLE})
endfunction()

function(rvbl_generate_machine_documentation)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "MODEL;PATH" "DEPENDENCIES")
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  # Model documentation
  add_custom_command(
    OUTPUT ${arg_PATH}/model.adoc ${arg_PATH}/model.html
    COMMAND
      ${GENERATOR_ASCIIDOC_MACHINE} --input ${arg_MODEL} --meta-model
      ${rvbl_root}/quintauris/meta/machine.schema.json --search-path
      ${rvbl_root}
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
endfunction()

function(rvbl_interface_library)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "NAME" "SOURCES;INCLUDES;LIBRARIES")

  add_library(rvbl_${arg_NAME} INTERFACE ${arg_SOURCES})
  target_include_directories(rvbl_${arg_NAME} INTERFACE ${arg_INCLUDES})
  target_link_libraries(
    rvbl_${arg_NAME} INTERFACE $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)

  rvbl_library_documentation(${arg_NAME})
endfunction()

function(rvbl_static_library)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "NAME" "SOURCES;INCLUDES;LIBRARIES")

  add_library(rvbl_${arg_NAME} STATIC ${arg_SOURCES})
  target_include_directories(rvbl_${arg_NAME} PUBLIC ${arg_INCLUDES})
  target_link_libraries(rvbl_${arg_NAME}
                        PUBLIC $<LIST:TRANSFORM,${arg_LIBRARIES},PREPEND,rvbl_>)

  rvbl_library_documentation(${arg_NAME})
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
  endif()

  if(arg_TEST)
    if("qemu" IN_LIST rvbl_runners)
      set(test_list "")

      add_test(
        NAME qemu.${arg_NAME}
        COMMAND
          ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker}
          --machine ${rvbl_machine_moniker} --runner qemu.system32-elf --file
          ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}${CMAKE_EXECUTABLE_SUFFIX_C})
      list(APPEND test_list qemu.${arg_NAME})

      if(arg_BINARY)
        add_test(
          NAME qemu.${arg_NAME}.bin
          COMMAND
            ${rvbl_rave_local} run --toolchain ${rvbl_toolchain_moniker}
            --machine ${rvbl_machine_moniker} --runner qemu.system32-bin --file
            ${CMAKE_CURRENT_BINARY_DIR}/${arg_NAME}.bin)
        list(APPEND test_list qemu.${arg_NAME}.bin)
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
          --machine ${rvbl_machine_moniker} --runner segger.jrun --dump-file
          ${CMAKE_CURRENT_BINARY_DIR}/:tt --file
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
      add_custom_command(
        TARGET ${flash_executable_name}
        POST_BUILD
        COMMAND
          ${OBJCOPY} $<TARGET_FILE:${flash_executable_name}>
          ${rvbl_elf_copy_parameters}
          ${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}.bin
        BYPRODUCTS ${CMAKE_CURRENT_BINARY_DIR}/${flash_executable_name}.bin
        COMMENT "Converting $<TARGET_FILE:${flash_executable_name}> to binary \
image...")
    endif()
  endif()
endfunction()

function(rvbl_test)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "NAME;TIMEOUT;EXPECTED_OUTPUT"
                        "LIBRARIES")

  rvbl_executable(
    NAME
    test_${arg_NAME}
    SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/tests/${arg_NAME}.c
    LIBRARIES
    test
    ${arg_LIBRARIES}
    TEST)
endfunction()

macro(rvbl_use)
  get_property(rvbl_root GLOBAL PROPERTY rvbl_root)

  foreach(parameter ${ARGV})
    add_subdirectory(${rvbl_root}/${parameter}
                     ${CMAKE_CURRENT_BINARY_DIR}/${parameter})
  endforeach()
endmacro()

function(rvbl_layer_documentation)
  cmake_parse_arguments(PARSE_ARGV 0 arg "" "SOURCE_DIR;TARGET_DIR;RVBL_ROOT"
                        "")

  make_directory(${arg_TARGET_DIR})
  set(temp_dir ${arg_SOURCE_DIR}/generated)
  make_directory(${temp_dir})

  find_program(ASCIIDOCTOR NAMES asciidoctor REQUIRED)
  find_program(EXTRACTOR_ASCIIDOC NAMES asciidoc_extractor REQUIRED)
  find_program(GENERATOR_ASCIIDOC_DEVICE NAMES generator_asciidoc_device
                                               REQUIRED)

  # Libraries
  file(
    GLOB_RECURSE inputs
    LIST_DIRECTORIES false
    RELATIVE ${CMAKE_CURRENT_SOURCE_DIR}
    "../lib/*.h")
  list(SORT inputs)
  set(temp_api ${temp_dir}/api.adoc)
  set(outputs_api ${arg_TARGET_DIR}/api.html)

  add_custom_command(
    OUTPUT ${outputs_api}
    COMMAND ${EXTRACTOR_ASCIIDOC} ${inputs} ${temp_api}
    COMMAND ${ASCIIDOCTOR} --doctype book -D ${arg_TARGET_DIR} ${temp_api}
    COMMAND_EXPAND_LISTS
    DEPENDS ${inputs}
    WORKING_DIRECTORY ${arg_SOURCE_DIR}
    COMMENT "Generating API documentation...")

  # Models
  file(
    GLOB_RECURSE inputs
    LIST_DIRECTORIES false
    "../model/*.json")
  set(outputs_model "")

  foreach(input ${inputs})
    get_filename_component(model_name ${input} DIRECTORY)
    get_filename_component(model_name ${model_name} NAME)
    set(output_adoc ${temp_dir}/${model_name}.adoc)
    set(output_html ${arg_TARGET_DIR}/${model_name}.html)
    list(APPEND outputs_model ${output_html})

    add_custom_command(
      OUTPUT ${output_html}
      COMMAND
        ${GENERATOR_ASCIIDOC_DEVICE} --input ${input} --meta-model
        ${arg_RVBL_ROOT}/quintauris/meta/device.schema.json --search-path
        ${arg_RVBL_ROOT}
      COMMAND ${CMAKE_COMMAND} -E rename ${temp_dir}/model.adoc ${output_adoc}
      COMMAND ${ASCIIDOCTOR} --doctype book -D ${arg_TARGET_DIR} ${output_adoc}
      DEPENDS ${input}
      WORKING_DIRECTORY ${temp_dir}
      COMMENT "Generating model ${model_name} documentation...")
  endforeach()

  # Main documents and common files
  file(GLOB inputs "${arg_SOURCE_DIR}/*.adoc")
  set(outputs_doc "")

  foreach(input ${inputs})
    get_filename_component(file_name_we ${input} NAME_WE)
    list(APPEND outputs_doc ${arg_TARGET_DIR}/${file_name_we}.html)
  endforeach()

  add_custom_command(
    OUTPUT ${outputs_doc}
    COMMAND ${ASCIIDOCTOR} --doctype book -D ${arg_TARGET_DIR} ${inputs}
    DEPENDS ${inputs} ${outputs_api} ${outputs_model}
    COMMENT "Generating documentation...")

  file(GLOB glob "${arg_SOURCE_DIR}/*.html" "${arg_SOURCE_DIR}/*.png")
  file(COPY ${glob} DESTINATION ${arg_TARGET_DIR})

  add_custom_target(docs ALL DEPENDS ${outputs_doc} ${outputs_api}
                                     ${outputs_model})
endfunction()
