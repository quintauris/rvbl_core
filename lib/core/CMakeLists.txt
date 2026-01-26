rvbl_static_library(
  NAME
  core
  SOURCES
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/core/rvbl_types.h"
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/core/rvbl_abi.h"
  "${CMAKE_CURRENT_SOURCE_DIR}/source/\
rvbl_abi${rvbl_toolchain_specific_suffix}.s"
  INCLUDES
  "${CMAKE_CURRENT_SOURCE_DIR}/include"
  LIBRARIES
  compiler)

rvbl_test(NAME global_pointer LIBRARIES core)
rvbl_test(NAME stack_pointer LIBRARIES core)
rvbl_test(NAME thread_pointer LIBRARIES core)
rvbl_test(NAME system_call LIBRARIES core)
