#[[
  Copyright 2026 Quintauris GmbH
  Licensed under the Apache License, Version 2.0 (the "License").
  https://www.apache.org/licenses/LICENSE-2.0
]]

rvbl_static_library(
  NAME
  c
  SOURCES
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/c/assert.h"
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/c/stddef.h"
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/c/stdint.h"
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/c/stdlib.h"
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/c/string.h"
  "${CMAKE_CURRENT_LIST_DIR}/source/rvbl_string.c"
  "${CMAKE_CURRENT_LIST_DIR}/source/rvbl_stdlib.c"
  INCLUDES
  "${CMAKE_CURRENT_LIST_DIR}/include"
  "${CMAKE_CURRENT_LIST_DIR}/include/rvbl/c"
  LIBRARIES
  compiler
  type
  alloc_bump)

rvbl_test(NAME string LIBRARIES c)
rvbl_test(NAME memset LIBRARIES c)
rvbl_test(NAME memcpy LIBRARIES c)
rvbl_test(NAME stdlib LIBRARIES c)
