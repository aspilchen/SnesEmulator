

set(SRC_FILES
  ricoh5a22.cc
  ricoh5a22_ops.cc
  ricoh5a22_addr.cc
  cpu_state.cc
  alu.cc
  snes_mem.cc)


add_library(emulator ${SRC_FILES})

target_include_directories(emulator PUBLIC ../include)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST})
