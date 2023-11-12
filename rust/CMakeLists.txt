externalproject_add(
  "rust"
  SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/src"
  BINARY_DIR "${CMAKE_CURRENT_SOURCE_DIR}/target/debug"
  CONFIGURE_COMMAND ""
  BUILD_COMMAND cargo build COMMAND cargo build --release
  # Trigger a build operation every recompile instead of having to manually keep a list of source files...
  # see if this is too slow
  UPDATE_COMMAND cargo build
  INSTALL_COMMAND ""
  TEST_COMMAND ""
)

# https://stackoverflow.com/questions/6351609/cmake-linking-to-library-downloaded-from-externalproject-add
SET(RUST_INCLUDE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target/cxxbridge)

# todo: debug and release ver?
SET(RUST_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target/debug)

add_library(rustlib SHARED IMPORTED)
set_target_properties(rustlib PROPERTIES IMPORTED_LOCATION ${RUST_LIB_DIR}/libarrowindex_db.a)