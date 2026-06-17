#[[
  Copyright 2026 Quintauris GmbH
  Licensed under the Apache License, Version 2.0 (the "License").
  https://www.apache.org/licenses/LICENSE-2.0
]]

rvbl_interface_library(
  NAME
  type
  SOURCES
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/type/rvbl_types.h"
  INCLUDES
  "${CMAKE_CURRENT_SOURCE_DIR}/include"
  LIBRARIES
  compiler)
