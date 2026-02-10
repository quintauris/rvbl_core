cmake_minimum_required(VERSION 3.28)
project(base_doc)

find_program(ASCIIDOCTOR NAMES asciidoctor REQUIRED)

set(input_dir ${CMAKE_CURRENT_SOURCE_DIR})
set(output_dir ${input_dir}/html)
set(input ${input_dir}/index.adoc)
set(output ${output_dir}/index.html)

make_directory(${output_dir})

add_custom_command(
  OUTPUT ${output}
  COMMAND ${ASCIIDOCTOR} ${input} -o ${output}
  WORKING_DIRECTORY ${input_dir}
  DEPENDS ${input}
  COMMENT "Generating documentation from ${input} ...")

add_custom_target(index ALL DEPENDS ${output})

file(GLOB glob "${input_dir}/*.html" "${input_dir}/*.png")
file(COPY ${glob} DESTINATION ${output_dir})
