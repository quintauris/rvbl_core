cmake_minimum_required(VERSION 3.28)
project(core_doc)

include(../cmake/support.cmake)

rvbl_layer_documentation(
  SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR} TARGET_DIR
  ${CMAKE_CURRENT_SOURCE_DIR}/html RVBL_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../..)
