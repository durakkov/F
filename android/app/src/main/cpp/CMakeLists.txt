cmake_minimum_required(VERSION 3.22)
project(woxel_android_bridge)

add_library(woxel_core SHARED
  ../../../../core_cpp/src/woxel_api.cpp
  ../../../../core_cpp/src/data/file_ops.cpp
  ../../../../core_cpp/src/data/search_index.cpp
  ../../../../core_cpp/src/domain/file_category.cpp
  ../../../../core_cpp/src/thumbnail/thumb_cache.cpp
)

target_include_directories(woxel_core PRIVATE
  ../../../../core_cpp/include
)
