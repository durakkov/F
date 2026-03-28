use chrono::Utc;
use sqlx::{Pool, Postgres, Sqlite};

use crate::db::models::{ChatSettings, DomainRule, Filter};

#[derive(Clone)]
pub enum DbPool {
    Sqlite(Pool<Sqlite>),
    Postgres(Pool<Postgres>),
}

pub async fn upsert_chat(pool: &DbPool, tg_chat_id: i64, title: &str) -> anyhow::Result<()> {
    match pool {
        DbPool::Sqlite(p) => {
            sqlx::query("INSERT INTO chats (tg_chat_id, title, created_at, updated_at) VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (tg_chat_id) DO UPDATE SET title = excluded.title, updated_at = CURRENT_TIMESTAMP")
                .bind(tg_chat_id)
                .bind(title)
                .execute(p)
                .await?;
        }
        DbPool::Postgres(p) => {
            sqlx::query("INSERT INTO chats (tg_chat_id, title, created_at, updated_at) VALUES ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (tg_chat_id) DO UPDATE SET title = excluded.title, updated_at = CURRENT_TIMESTAMP")
                .bind(tg_chat_id)
                .bind(title)
                .execute(p)
                .await?;
        }
    }
    Ok(())
}

pub async fn upsert_user(
    pool: &DbPool,
    user_id: i64,
    username: Option<&str>,
) -> anyhow::Result<()> {
    match pool {
        DbPool::Sqlite(p) => {
            sqlx::query("INSERT INTO users (tg_user_id, username, first_seen, last_seen) VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (tg_user_id) DO UPDATE SET username = excluded.username, last_seen = CURRENT_TIMESTAMP")
                .bind(user_id)
                .bind(username)
                .execute(p)
                .await?;
        }
        DbPool::Postgres(p) => {
            sqlx::query("INSERT INTO users (tg_user_id, username, first_seen, last_seen) VALUES ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT (tg_user_id) DO UPDATE SET username = excluded.username, last_seen = CURRENT_TIMESTAMP")
                .bind(user_id)
                .bind(username)
                .execute(p)
                .await?;
        }
    }
    Ok(())
}

pub async fn ensure_chat_settings(pool: &DbPool, tg_chat_id: i64) -> anyhow::Result<()> {
    match pool {
        DbPool::Sqlite(p) => {
            sqlx::query("INSERT INTO chat_settings (tg_chat_id) VALUES (?) ON CONFLICT (tg_chat_id) DO NOTHING")
                .bind(tg_chat_id)
                .execute(p)
                .await?;
        }
        DbPool::Postgres(p) => {
            sqlx::query("INSERT INTO chat_settings (tg_chat_id) VALUES ($1) ON CONFLICT (tg_chat_id) DO NOTHING")
                .bind(tg_chat_id)
                .execute(p)
                .await?;
        }
    }
    Ok(())
}

pub async fn get_chat_settings(pool: &DbPool, tg_chat_id: i64) -> anyhow::Result<ChatSettings> {
    match pool {
        DbPool::Sqlite(p) => Ok(sqlx::query_as::<_, ChatSettings>(
            "SELECT * FROM chat_settings WHERE tg_chat_id = ?",
        )
        .bind(tg_chat_id)
        .fetch_one(p)
        .await?),
        DbPool::Postgres(p) => Ok(sqlx::query_as::<_, ChatSettings>(
            "SELECT * FROM chat_settings WHERE tg_chat_id = $1",
        )
        .bind(tg_chat_id)
        .fetch_one(p)
        .await?),
    }
}

pub async fn increment_warn(
    pool: &DbPool,
    tg_chat_id: i64,
    tg_user_id: i64,
) -> anyhow::Result<i32> {
    match pool {
        DbPool::Sqlite(p) => {
            sqlx::query("INSERT INTO chat_users (tg_chat_id, tg_user_id, warn_count, last_violation_at, whitelisted) VALUES (?, ?, 1, CURRENT_TIMESTAMP, false)
                         ON CONFLICT(tg_chat_id, tg_user_id) DO UPDATE SET warn_count = warn_count + 1, last_violation_at = CURRENT_TIMESTAMP")
                .bind(tg_chat_id)
                .bind(tg_user_id)
                .execute(p)
                .await?;
            Ok(sqlx::query_scalar(
                "SELECT warn_count FROM chat_users WHERE tg_chat_id = ? AND tg_user_id = ?",
            )
            .bind(tg_chat_id)
            .bind(tg_user_id)
            .fetch_one(p)
            .await?)
        }
        DbPool::Postgres(p) => {
            sqlx::query("INSERT INTO chat_users (tg_chat_id, tg_user_id, warn_count, last_violation_at, whitelisted) VALUES ($1, $2, 1, CURRENT_TIMESTAMP, false)
                         ON CONFLICT(tg_chat_id, tg_user_id) DO UPDATE SET warn_count = chat_users.warn_count + 1, last_violation_at = CURRENT_TIMESTAMP")
                .bind(tg_chat_id)
                .bind(tg_user_id)
                .execute(p)
                .await?;
            Ok(sqlx::query_scalar(
                "SELECT warn_count FROM chat_users WHERE tg_chat_id = $1 AND tg_user_id = $2",
            )
            .bind(tg_chat_id)
            .bind(tg_user_id)
            .fetch_one(p)
            .await?)
        }
    }
}

pub async fn load_filters(pool: &DbPool, tg_chat_id: i64) -> anyhow::Result<Vec<Filter>> {
    match pool {
        DbPool::Sqlite(p) => Ok(sqlx::query_as(
            "SELECT * FROM filters WHERE tg_chat_id = ? AND enabled = 1",
        )
        .bind(tg_chat_id)
        .fetch_all(p)
        .await?),
        DbPool::Postgres(p) => Ok(sqlx::query_as(
            "SELECT * FROM filters WHERE tg_chat_id = $1 AND enabled = true",
        )
        .bind(tg_chat_id)
        .fetch_all(p)
        .await?),
    }
}

pub async fn load_domains(pool: &DbPool, tg_chat_id: i64) -> anyhow::Result<Vec<DomainRule>> {
    match pool {
        DbPool::Sqlite(p) => Ok(sqlx::query_as(
            "SELECT * FROM domains WHERE tg_chat_id = ? AND enabled = 1",
        )
        .bind(tg_chat_id)
        .fetch_all(p)
        .await?),
        DbPool::Postgres(p) => Ok(sqlx::query_as(
            "SELECT * FROM domains WHERE tg_chat_id = $1 AND enabled = true",
        )
        .bind(tg_chat_id)
        .fetch_all(p)
        .await?),
    }
}

pub async fn log_action(
    pool: &DbPool,
    chat_id: i64,
    user_id: i64,
    action: &str,
    reason: &str,
    actor_user_id: Option<i64>,
) -> anyhow::Result<()> {
    let created = Utc::now();
    match pool {
        DbPool::Sqlite(p) => {
            sqlx::query("INSERT INTO mod_actions (tg_chat_id, tg_user_id, action, reason, created_at, actor_user_id) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(chat_id)
                .bind(user_id)
                .bind(action)
                .bind(reason)
                .bind(created)
                .bind(actor_user_id)
                .execute(p)
                .await?;
        }
        DbPool::Postgres(p) => {
            sqlx::query("INSERT INTO mod_actions (tg_chat_id, tg_user_id, action, reason, created_at, actor_user_id) VALUES ($1, $2, $3, $4, $5, $6)")
                .bind(chat_id)
                .bind(user_id)
                .bind(action)
                .bind(reason)
                .bind(created)
                .bind(actor_user_id)
                .execute(p)
                .await?;
        }
    }
    Ok(())
}
