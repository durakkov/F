#pragma once
#include <cstdint>
#include <string>
#include <vector>
#include "woxel/error.h"

namespace woxel {

struct FileEntry {
  std::string path;
  std::string name;
  uintmax_t size = 0;
  int64_t modified_epoch_ms = 0;
  bool is_dir = false;
};

Error ListDirectory(const std::string& path, std::vector<FileEntry>* out_entries);
Error CopyPath(const std::string& src, const std::string& dst, bool overwrite);
Error MovePath(const std::string& src, const std::string& dst, bool overwrite);
Error DeletePath(const std::string& path);
Error RenamePath(const std::string& path, const std::string& new_name);
Error CreateFolder(const std::string& path);

}  // namespace woxel
