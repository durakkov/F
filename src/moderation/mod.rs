pub mod rules;

use teloxide::prelude::*;
use teloxide::types::{ChatPermissions, UserId};

use crate::db::queries::{
    get_chat_settings, increment_warn, load_domains, load_filters, log_action, DbPool,
};
use crate::moderation::rules::{check_filters, check_links, FloodTracker, ViolationKind};

pub struct ModerationEngine {
    flood: tokio::sync::Mutex<FloodTracker>,
}

impl ModerationEngine {
    pub fn new() -> Self {
        Self {
            flood: tokio::sync::Mutex::new(FloodTracker::new()),
        }
    }

    pub async fn moderate_message(
        &self,
        bot: &Bot,
        db: &DbPool,
        msg: &Message,
    ) -> anyhow::Result<Option<ViolationKind>> {
        let chat_id = msg.chat.id.0;
        let user = match msg.from() {
            Some(u) => u,
            None => return Ok(None),
        };
        let text = msg.text().unwrap_or_default();
        if text.is_empty() {
            return Ok(None);
        }

        let settings = get_chat_settings(db, chat_id).await?;
        if !settings.moderation_enabled {
            return Ok(None);
        }

        let filters = load_filters(db, chat_id).await?;
        if check_filters(text, &filters) {
            self.enforce(
                bot,
                db,
                msg,
                user.id,
                "filtered",
                "Запрещённый контент",
                &settings.default_sanction,
                settings.mute_minutes,
            )
            .await?;
            return Ok(Some(ViolationKind::Filtered));
        }

        let domains = load_domains(db, chat_id).await?;
        if check_links(text, &settings, &domains) {
            self.enforce(
                bot,
                db,
                msg,
                user.id,
                "link",
                "Нарушение политики ссылок",
                &settings.default_sanction,
                settings.mute_minutes,
            )
            .await?;
            return Ok(Some(ViolationKind::LinkDenied));
        }

        let flooded = self.flood.lock().await.hit(
            chat_id,
            user.id.0 as i64,
            settings.flood_window_sec,
            settings.flood_max_msgs,
        );
        if flooded {
            self.enforce(
                bot,
                db,
                msg,
                user.id,
                "flood",
                "Антифлуд",
                "mute",
                settings.mute_minutes,
            )
            .await?;
            return Ok(Some(ViolationKind::Flood));
        }

        Ok(None)
    }

    async fn enforce(
        &self,
        bot: &Bot,
        db: &DbPool,
        msg: &Message,
        offender: UserId,
        action: &str,
        reason: &str,
        default_sanction: &str,
        mute_minutes: i32,
    ) -> anyhow::Result<()> {
        let chat_id = msg.chat.id;
        let _ = bot.delete_message(chat_id, msg.id).await;
        let warns = increment_warn(db, chat_id.0, offender.0 as i64).await?;

        if default_sanction == "mute" || action == "flood" {
            let until = chrono::Utc::now() + chrono::Duration::minutes(mute_minutes as i64);
            let permissions = ChatPermissions::empty();
            let _ = bot
                .restrict_chat_member(chat_id, offender, permissions)
                .until_date(until)
                .await;
        }

        if default_sanction == "ban" {
            let _ = bot.ban_chat_member(chat_id, offender).await;
        }

        log_action(db, chat_id.0, offender.0 as i64, action, reason, None).await?;
        let _ = bot
            .send_message(
                chat_id,
                format!("⚠️ Пользователь получил предупреждение ({warns}) — {reason}"),
            )
            .await;
        Ok(())
    }
}
