rvbl_static_library(
  NAME
  alloc
  SOURCES
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/alloc/rvbl_alloc.h"
  "${CMAKE_CURRENT_LIST_DIR}/source/rvbl_alloc.c"
  INCLUDES
  "${CMAKE_CURRENT_LIST_DIR}/include"
  LIBRARIES
  compiler
  core)

rvbl_test(NAME alloc LIBRARIES alloc)

rvbl_static_library(
  NAME
  alloc_bump
  SOURCES
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/alloc/rvbl_alloc_bump.h"
  "${CMAKE_CURRENT_LIST_DIR}/source/rvbl_alloc_bump.c"
  INCLUDES
  "${CMAKE_CURRENT_LIST_DIR}/include"
  LIBRARIES
  alloc)

rvbl_test(NAME bump LIBRARIES alloc_bump)
