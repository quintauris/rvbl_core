cmake_minimum_required(VERSION 3.28)
project(core_doc)

find_program(ASCIIDOCTOR NAMES asciidoctor REQUIRED)

set(input_dir ${CMAKE_CURRENT_SOURCE_DIR})
set(output_dir ${input_dir}/html)

file(GLOB inputs "${input_dir}/*.adoc")
set(outputs ${inputs})
list(TRANSFORM outputs REPLACE ".adoc" ".html")

make_directory(${output_dir})

add_custom_command(
  OUTPUT ${outputs}
  COMMAND ${ASCIIDOCTOR} -D ${output_dir} ${inputs}
  WORKING_DIRECTORY ${input_dir}
  DEPENDS ${inputs}
  COMMENT "Generating documentation...")

add_custom_target(index ALL DEPENDS ${outputs})

file(GLOB glob "${input_dir}/*.html" "${input_dir}/*.png")
file(COPY ${glob} DESTINATION ${output_dir})
