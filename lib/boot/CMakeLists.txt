#[[
  Copyright 2026 Quintauris GmbH
  Licensed under the Apache License, Version 2.0 (the "License").
  https://www.apache.org/licenses/LICENSE-2.0
]]

rvbl_static_library(
  NAME
  boot
  SOURCES
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/boot/rvbl_boot.h"
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/boot/rvbl_extension.h"
  "${CMAKE_CURRENT_SOURCE_DIR}/source/init${rvbl_toolchain_specific_suffix}.s"
  "${CMAKE_CURRENT_SOURCE_DIR}/source/rvbl_boot.c"
  "${CMAKE_CURRENT_SOURCE_DIR}/source/rvbl_extension.c"
  INCLUDES
  "${CMAKE_CURRENT_SOURCE_DIR}/include"
  LIBRARIES
  machine)
