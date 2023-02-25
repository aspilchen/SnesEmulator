add_library(hardware STATIC Ricoh5A22/Ricoh5A22.cpp Ricoh5A22/Ricoh5A22.hpp)
target_include_directories(hardware PUBLIC Ricoh5A22)
target_link_libraries(hardware PRIVATE pthread)
target_compile_features(hardware PUBLIC cxx_std_17)