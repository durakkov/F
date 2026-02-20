#include <cassert>
#include "woxel/file_category.h"

int main() {
  assert(woxel::DetectCategory("a.jpg") == woxel::FileCategory::Images);
  assert(woxel::DetectCategory("a.pdf") == woxel::FileCategory::Documents);
  assert(woxel::DetectCategory("a.zip") == woxel::FileCategory::Archives);
  return 0;
}
