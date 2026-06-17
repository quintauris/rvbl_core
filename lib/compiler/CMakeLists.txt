#[[
  Copyright 2026 Quintauris GmbH
  Licensed under the Apache License, Version 2.0 (the "License").
  https://www.apache.org/licenses/LICENSE-2.0
]]

rvbl_static_library(
  NAME
  compiler
  SOURCES
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/compiler/rvbl_compiler.h"
  "${CMAKE_CURRENT_SOURCE_DIR}/source/rvbl_arith64.c"
  INCLUDES
  "${CMAKE_CURRENT_SOURCE_DIR}/include")
