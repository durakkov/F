#pragma once
#include <mutex>
#include <string>
#include <unordered_set>

namespace woxel {

class ThumbCache {
 public:
  bool MarkInFlight(const std::string& key);
  void Complete(const std::string& key);

 private:
  std::mutex mu_;
  std::unordered_set<std::string> in_flight_;
};

}  // namespace woxel
