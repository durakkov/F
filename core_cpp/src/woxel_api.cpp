#include <cstring>
#include <string>
#include <vector>

#include "woxel/file_category.h"
#include "woxel/file_ops.h"

extern "C" {

struct WoxelEntry {
  char path[1024];
  char name[256];
  unsigned long long size;
  long long modified_ms;
  int is_dir;
  int category;
};

int woxel_list_dir(const char* path, WoxelEntry* entries, int max_entries) {
  std::vector<woxel::FileEntry> data;
  auto err = woxel::ListDirectory(path, &data);
  if (err) return -err.code;
  int n = static_cast<int>(data.size());
  if (n > max_entries) n = max_entries;
  for (int i = 0; i < n; ++i) {
    std::strncpy(entries[i].path, data[i].path.c_str(), sizeof(entries[i].path) - 1);
    std::strncpy(entries[i].name, data[i].name.c_str(), sizeof(entries[i].name) - 1);
    entries[i].size = data[i].size;
    entries[i].modified_ms = data[i].modified_epoch_ms;
    entries[i].is_dir = data[i].is_dir ? 1 : 0;
    entries[i].category = static_cast<int>(woxel::DetectCategory(data[i].path));
  }
  return n;
}

int woxel_copy(const char* src, const char* dst, int overwrite) { return woxel::CopyPath(src, dst, overwrite).code; }
int woxel_move(const char* src, const char* dst, int overwrite) { return woxel::MovePath(src, dst, overwrite).code; }
int woxel_delete(const char* path) { return woxel::DeletePath(path).code; }
int woxel_mkdir(const char* path) { return woxel::CreateFolder(path).code; }

}
