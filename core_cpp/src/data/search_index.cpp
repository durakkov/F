#include "woxel/search_index.h"

#include <sqlite3.h>

namespace woxel {

SearchIndex::SearchIndex(std::string db_path) : db_path_(std::move(db_path)) {}

bool SearchIndex::Init() {
  sqlite3* db = nullptr;
  if (sqlite3_open(db_path_.c_str(), &db) != SQLITE_OK) return false;
  const char* sql = "CREATE TABLE IF NOT EXISTS file_index(path TEXT PRIMARY KEY, name TEXT, modified_ms INTEGER, size INTEGER);"
                    "CREATE INDEX IF NOT EXISTS idx_name ON file_index(name);";
  const bool ok = sqlite3_exec(db, sql, nullptr, nullptr, nullptr) == SQLITE_OK;
  sqlite3_close(db);
  return ok;
}

bool SearchIndex::Upsert(const std::string& path, const std::string& name, int64_t modified_ms, int64_t size) {
  sqlite3* db = nullptr;
  if (sqlite3_open(db_path_.c_str(), &db) != SQLITE_OK) return false;
  const char* sql = "INSERT INTO file_index(path,name,modified_ms,size) VALUES(?,?,?,?) "
                    "ON CONFLICT(path) DO UPDATE SET name=excluded.name, modified_ms=excluded.modified_ms, size=excluded.size;";
  sqlite3_stmt* stmt = nullptr;
  sqlite3_prepare_v2(db, sql, -1, &stmt, nullptr);
  sqlite3_bind_text(stmt, 1, path.c_str(), -1, SQLITE_TRANSIENT);
  sqlite3_bind_text(stmt, 2, name.c_str(), -1, SQLITE_TRANSIENT);
  sqlite3_bind_int64(stmt, 3, modified_ms);
  sqlite3_bind_int64(stmt, 4, size);
  const bool ok = sqlite3_step(stmt) == SQLITE_DONE;
  sqlite3_finalize(stmt);
  sqlite3_close(db);
  return ok;
}

std::vector<SearchResult> SearchIndex::Query(const std::string& text, int limit) const {
  std::vector<SearchResult> out;
  sqlite3* db = nullptr;
  if (sqlite3_open(db_path_.c_str(), &db) != SQLITE_OK) return out;
  const char* sql = "SELECT path,name FROM file_index WHERE name LIKE ? ORDER BY modified_ms DESC LIMIT ?;";
  sqlite3_stmt* stmt = nullptr;
  sqlite3_prepare_v2(db, sql, -1, &stmt, nullptr);
  std::string pattern = "%" + text + "%";
  sqlite3_bind_text(stmt, 1, pattern.c_str(), -1, SQLITE_TRANSIENT);
  sqlite3_bind_int(stmt, 2, limit);
  while (sqlite3_step(stmt) == SQLITE_ROW) {
    out.push_back({reinterpret_cast<const char*>(sqlite3_column_text(stmt, 0)), reinterpret_cast<const char*>(sqlite3_column_text(stmt, 1))});
  }
  sqlite3_finalize(stmt);
  sqlite3_close(db);
  return out;
}

}  // namespace woxel
