rvbl_static_library(
  NAME
  semihost
  SOURCES
  "${CMAKE_CURRENT_LIST_DIR}/source/\
rvbl_semihost${rvbl_toolchain_specific_suffix}.s"
  "${CMAKE_CURRENT_LIST_DIR}/source/rvbl_semihost.c"
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/semihost/rvbl_semihost.h"
  INCLUDES
  "${CMAKE_CURRENT_LIST_DIR}/include"
  LIBRARIES
  core
  c)

rvbl_test(NAME semihost LIBRARIES semihost)
