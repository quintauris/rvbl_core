rvbl_static_library(
  NAME
  uart
  SOURCES
  "${CMAKE_CURRENT_SOURCE_DIR}/source/rvbl_uart.c"
  "${CMAKE_CURRENT_SOURCE_DIR}/include/rvbl/uart/rvbl_uart.h"
  INCLUDES
  "${CMAKE_CURRENT_LIST_DIR}/include"
  LIBRARIES
  type)

rvbl_test(NAME uart_buffered LIBRARIES uart)
