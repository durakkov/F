use std::collections::HashMap;
use std::time::{Duration, Instant};

use teloxide::prelude::*;
use teloxide::types::{ChatMemberStatus, UserId};

use crate::config::Config;

#[derive(Default)]
pub struct CallbackRateLimit {
    hits: tokio::sync::Mutex<HashMap<i64, Vec<Instant>>>,
}

impl CallbackRateLimit {
    pub async fn allow(&self, user_id: i64, per_sec: u32) -> bool {
        let mut guard = self.hits.lock().await;
        let now = Instant::now();
        let bucket = guard.entry(user_id).or_default();
        bucket.push(now);
        bucket.retain(|t| now.duration_since(*t) <= Duration::from_secs(1));
        bucket.len() <= per_sec as usize
    }
}

pub async fn is_admin_or_superadmin(
    bot: &Bot,
    cfg: &Config,
    chat_id: ChatId,
    user_id: UserId,
) -> bool {
    if cfg.admin_ids.iter().any(|id| *id == user_id.0 as i64) {
        return true;
    }
    match bot.get_chat_member(chat_id, user_id).await {
        Ok(member) => matches!(
            member.status(),
            ChatMemberStatus::Administrator | ChatMemberStatus::Owner
        ),
        Err(_) => false,
    }
}
