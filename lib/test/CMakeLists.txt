#[[
  Copyright 2026 Quintauris GmbH
  Licensed under the Apache License, Version 2.0 (the "License").
  https://www.apache.org/licenses/LICENSE-2.0
]]

get_property(rvbl_machine_dir GLOBAL PROPERTY rvbl_machine_dir)

rvbl_static_library(
  NAME
  test
  SOURCES
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/test/rvbl_test.h"
  "${CMAKE_CURRENT_LIST_DIR}/source/\
rvbl_test${rvbl_toolchain_specific_suffix}.s"
  "${CMAKE_CURRENT_LIST_DIR}/source/rvbl_test_portable.c"
  "${rvbl_machine_dir}/source/rvbl_test.c"
  INCLUDES
  "${CMAKE_CURRENT_SOURCE_DIR}/../../third_party/nanoprintf"
  "${CMAKE_CURRENT_LIST_DIR}/include"
  "${CMAKE_CURRENT_BINARY_DIR}/../../machine/${machine}/generated"
  LIBRARIES
  type
  machine
  c
  semihost
  alloc_bump)
