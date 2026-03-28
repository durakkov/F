use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct ChatSettings {
    pub tg_chat_id: i64,
    pub language: String,
    pub moderation_enabled: bool,
    pub links_mode: String,
    pub flood_window_sec: i32,
    pub flood_max_msgs: i32,
    pub warn_threshold: i32,
    pub default_sanction: String,
    pub mute_minutes: i32,
    pub raid_window_sec: i32,
    pub raid_max_joins: i32,
    pub lockdown_minutes: i32,
    pub verification_enabled: bool,
    pub verification_timeout_min: i32,
    pub log_chat_id: Option<i64>,
    pub log_thread_id: Option<i64>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Filter {
    pub id: i64,
    pub tg_chat_id: i64,
    pub kind: String,
    pub pattern: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DomainRule {
    pub id: i64,
    pub tg_chat_id: i64,
    pub list_type: String,
    pub domain: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ChatUser {
    pub tg_chat_id: i64,
    pub tg_user_id: i64,
    pub warn_count: i32,
    pub last_violation_at: Option<DateTime<Utc>>,
    pub whitelisted: bool,
}
