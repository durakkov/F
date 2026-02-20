#include "woxel/file_category.h"

#include <algorithm>
#include <fstream>
#include <unordered_map>

namespace woxel {
namespace {
std::string ToLower(std::string s) {
  std::transform(s.begin(), s.end(), s.begin(), [](unsigned char c) { return std::tolower(c); });
  return s;
}

std::string Ext(const std::string& p) {
  auto pos = p.find_last_of('.');
  if (pos == std::string::npos) return "";
  return ToLower(p.substr(pos + 1));
}

bool HasMagic(const std::string& path, const std::string& magic) {
  std::ifstream f(path, std::ios::binary);
  if (!f) return false;
  std::string buf(magic.size(), '\0');
  f.read(buf.data(), static_cast<std::streamsize>(magic.size()));
  return buf == magic;
}
}  // namespace

FileCategory DetectCategory(const std::string& path, const std::string& mime_hint) {
  const std::string mime = ToLower(mime_hint);
  if (mime.rfind("image/", 0) == 0) return FileCategory::Images;
  if (mime.rfind("video/", 0) == 0) return FileCategory::Videos;
  if (mime.rfind("audio/", 0) == 0) return FileCategory::Audio;
  if (mime == "application/vnd.android.package-archive") return FileCategory::Apk;
  if (mime == "application/pdf" || mime.find("document") != std::string::npos) return FileCategory::Documents;

  const std::string ext = Ext(path);
  static const std::unordered_map<std::string, FileCategory> kMap = {
      {"jpg", FileCategory::Images}, {"jpeg", FileCategory::Images}, {"png", FileCategory::Images},
      {"gif", FileCategory::Images}, {"webp", FileCategory::Images}, {"heic", FileCategory::Images},
      {"mp4", FileCategory::Videos}, {"mkv", FileCategory::Videos}, {"avi", FileCategory::Videos},
      {"mp3", FileCategory::Audio}, {"flac", FileCategory::Audio}, {"wav", FileCategory::Audio},
      {"pdf", FileCategory::Documents}, {"doc", FileCategory::Documents}, {"docx", FileCategory::Documents},
      {"zip", FileCategory::Archives}, {"rar", FileCategory::Archives}, {"7z", FileCategory::Archives},
      {"apk", FileCategory::Apk},
      {"cpp", FileCategory::Code}, {"h", FileCategory::Code}, {"kt", FileCategory::Code}, {"dart", FileCategory::Code}};

  auto it = kMap.find(ext);
  if (it != kMap.end()) return it->second;

  if (HasMagic(path, "\x89PNG")) return FileCategory::Images;
  if (HasMagic(path, "%PDF")) return FileCategory::Documents;
  if (HasMagic(path, "PK\x03\x04")) return FileCategory::Archives;

  return FileCategory::Others;
}

const char* ToString(FileCategory category) {
  switch (category) {
    case FileCategory::Images: return "images";
    case FileCategory::Videos: return "videos";
    case FileCategory::Audio: return "audio";
    case FileCategory::Documents: return "documents";
    case FileCategory::Archives: return "archives";
    case FileCategory::Apk: return "apk";
    case FileCategory::Code: return "code";
    default: return "others";
  }
}

}  // namespace woxel
