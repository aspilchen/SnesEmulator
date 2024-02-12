

set(SRC_FILES
  ricoh5a22.cc
  ricoh5a22.undefined.cc
  ricoh5a22.alu.cc
  ricoh5a22.lsi.cc
  ricoh5a22.addr.cc
  ricoh5a22.transfer.cc
  ricoh5a22.branch.cc
  ricoh5a22.jump.cc
  ricoh5a22.inturrupt.cc
  ricoh5a22.pflag.cc
  ricoh5a22.stack.cc
  cpu_state.cc
  alu.cc
  snes_mem.cc)


add_library(emulator ${SRC_FILES})

target_include_directories(emulator PUBLIC ../include)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST})
