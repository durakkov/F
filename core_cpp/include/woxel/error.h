#pragma once
#include <string>

namespace woxel {

enum class ErrorCategory { kNone, kPermission, kIo, kInvalidPath, kConflict, kUnknown };

struct Error {
  int code = 0;
  ErrorCategory category = ErrorCategory::kNone;
  std::string message;

  explicit operator bool() const { return code != 0; }
};

}  // namespace woxel
