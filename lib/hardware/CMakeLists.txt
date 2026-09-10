#[[
  Copyright 2026 Quintauris GmbH
  Licensed under the Apache License, Version 2.0 (the "License").
  https://www.apache.org/licenses/LICENSE-2.0
]]

rvbl_interface_library(
  NAME
  hardware
  SOURCES
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/hardware/rvbl_hardware.h"
  INCLUDES
  "${CMAKE_CURRENT_SOURCE_DIR}/include"
  LIBRARIES
  type)
