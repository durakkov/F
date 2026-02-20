#pragma once
#include <string>

namespace woxel {

enum class FileCategory {
  Images,
  Videos,
  Audio,
  Documents,
  Archives,
  Apk,
  Code,
  Others
};

FileCategory DetectCategory(const std::string& path, const std::string& mime_hint = "");
const char* ToString(FileCategory category);

}  // namespace woxel
