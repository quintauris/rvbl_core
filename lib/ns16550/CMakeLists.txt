#[[
  Copyright 2026 Quintauris GmbH
  Licensed under the Apache License, Version 2.0 (the "License").
  https://www.apache.org/licenses/LICENSE-2.0
]]

rvbl_static_library(
  NAME
  uart_ns16550
  SOURCES
  "${CMAKE_CURRENT_SOURCE_DIR}/source/rvbl_uart.c"
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/uart/rvbl_uart_ns16550.h"
  INCLUDES
  "${CMAKE_CURRENT_LIST_DIR}/include"
  LIBRARIES
  machine
  uart)

rvbl_test(NAME uart_ns16550 LIBRARIES uart_ns16550)
