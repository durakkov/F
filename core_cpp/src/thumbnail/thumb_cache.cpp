#include "woxel/thumb_cache.h"

namespace woxel {

bool ThumbCache::MarkInFlight(const std::string& key) {
  std::lock_guard<std::mutex> lock(mu_);
  if (in_flight_.contains(key)) return false;
  in_flight_.insert(key);
  return true;
}

void ThumbCache::Complete(const std::string& key) {
  std::lock_guard<std::mutex> lock(mu_);
  in_flight_.erase(key);
}

}  // namespace woxel
