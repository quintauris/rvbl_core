rvbl_static_library(
  NAME
  compiler
  SOURCES
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/compiler/rvbl_compiler.h"
  "${CMAKE_CURRENT_SOURCE_DIR}/source/rvbl_arith64.c"
  INCLUDES
  "${CMAKE_CURRENT_SOURCE_DIR}/include")
