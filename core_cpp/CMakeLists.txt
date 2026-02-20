cmake_minimum_required(VERSION 3.22)
project(woxel_core LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(woxel_core SHARED
  src/woxel_api.cpp
  src/data/file_ops.cpp
  src/data/search_index.cpp
  src/domain/file_category.cpp
  src/thumbnail/thumb_cache.cpp
)

target_include_directories(woxel_core PUBLIC include)
find_package(SQLite3 REQUIRED)
target_link_libraries(woxel_core PRIVATE SQLite::SQLite3)

add_executable(core_tests tests/core_tests.cpp src/domain/file_category.cpp)
target_include_directories(core_tests PRIVATE include)
add_test(NAME core_tests COMMAND core_tests)
enable_testing()
