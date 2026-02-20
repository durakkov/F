#pragma once
#include <string>
#include <vector>

namespace woxel {

struct SearchResult {
  std::string path;
  std::string name;
};

class SearchIndex {
 public:
  explicit SearchIndex(std::string db_path);
  bool Init();
  bool Upsert(const std::string& path, const std::string& name, int64_t modified_ms, int64_t size);
  std::vector<SearchResult> Query(const std::string& text, int limit) const;

 private:
  std::string db_path_;
};

}  // namespace woxel
