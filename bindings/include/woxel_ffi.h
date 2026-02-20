#pragma once
#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  char path[1024];
  char name[256];
  unsigned long long size;
  long long modified_ms;
  int is_dir;
  int category;
} WoxelEntry;

int woxel_list_dir(const char* path, WoxelEntry* entries, int max_entries);
int woxel_copy(const char* src, const char* dst, int overwrite);
int woxel_move(const char* src, const char* dst, int overwrite);
int woxel_delete(const char* path);
int woxel_mkdir(const char* path);

#ifdef __cplusplus
}
#endif
