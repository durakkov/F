#include "woxel/file_ops.h"

#include <chrono>
#include <filesystem>

namespace fs = std::filesystem;

namespace woxel {
namespace {
Error Err(int code, ErrorCategory category, std::string message) { return Error{code, category, std::move(message)}; }
bool IsSafe(const std::string& path) { return !path.empty() && path.find("..") == std::string::npos && path.find('\0') == std::string::npos; }
int64_t ToMs(fs::file_time_type t) {
  auto sctp = std::chrono::time_point_cast<std::chrono::milliseconds>(t - fs::file_time_type::clock::now() + std::chrono::system_clock::now());
  return sctp.time_since_epoch().count();
}
}  // namespace

Error ListDirectory(const std::string& path, std::vector<FileEntry>* out_entries) {
  if (!IsSafe(path)) return Err(1, ErrorCategory::kInvalidPath, "Unsafe path");
  try {
    out_entries->clear();
    for (const auto& e : fs::directory_iterator(path)) {
      FileEntry entry;
      entry.path = e.path().string();
      entry.name = e.path().filename().string();
      entry.is_dir = e.is_directory();
      entry.size = entry.is_dir ? 0 : e.file_size();
      entry.modified_epoch_ms = ToMs(e.last_write_time());
      out_entries->push_back(std::move(entry));
    }
    return {};
  } catch (const std::exception& ex) {
    return Err(2, ErrorCategory::kIo, ex.what());
  }
}

Error CopyPath(const std::string& src, const std::string& dst, bool overwrite) {
  if (!IsSafe(src) || !IsSafe(dst)) return Err(1, ErrorCategory::kInvalidPath, "Unsafe path");
  std::error_code ec;
  fs::copy(src, dst, overwrite ? fs::copy_options::overwrite_existing | fs::copy_options::recursive : fs::copy_options::recursive, ec);
  if (ec) return Err(ec.value(), ErrorCategory::kIo, ec.message());
  return {};
}
Error MovePath(const std::string& src, const std::string& dst, bool overwrite) {
  if (!overwrite && fs::exists(dst)) return Err(3, ErrorCategory::kConflict, "Destination exists");
  std::error_code ec;
  fs::rename(src, dst, ec);
  if (ec) return Err(ec.value(), ErrorCategory::kIo, ec.message());
  return {};
}
Error DeletePath(const std::string& path) {
  std::error_code ec;
  fs::remove_all(path, ec);
  if (ec) return Err(ec.value(), ErrorCategory::kIo, ec.message());
  return {};
}
Error RenamePath(const std::string& path, const std::string& new_name) {
  fs::path p(path);
  return MovePath(path, (p.parent_path() / new_name).string(), false);
}
Error CreateFolder(const std::string& path) {
  std::error_code ec;
  fs::create_directories(path, ec);
  if (ec) return Err(ec.value(), ErrorCategory::kIo, ec.message());
  return {};
}

}  // namespace woxel
