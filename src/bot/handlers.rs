use std::sync::Arc;

use teloxide::prelude::*;
use teloxide::types::{
    CallbackQuery, ChatKind, ChatMemberKind, ChatMemberUpdated, ChatPermissions,
    InlineKeyboardButton, InlineKeyboardMarkup, ParseMode,
};

use crate::bot::middleware::{is_admin_or_superadmin, CallbackRateLimit};
use crate::bot::ui::{group_panel_keyboard, private_main_menu};
use crate::config::Config;
use crate::db::queries::{ensure_chat_settings, upsert_chat, upsert_user, DbPool};
use crate::moderation::ModerationEngine;

#[derive(Clone)]
pub struct AppState {
    pub cfg: Config,
    pub db: DbPool,
    pub limiter: Arc<CallbackRateLimit>,
    pub moderation: Arc<ModerationEngine>,
    pub bot_name: String,
}

pub async fn on_message(bot: Bot, msg: Message, state: Arc<AppState>) -> anyhow::Result<()> {
    if let Some(user) = msg.from() {
        upsert_user(&state.db, user.id.0 as i64, user.username.as_deref()).await?;
    }

    match &msg.chat.kind {
        ChatKind::Public(_) => {
            upsert_chat(
                &state.db,
                msg.chat.id.0,
                msg.chat.title().unwrap_or("Untitled"),
            )
            .await?;
            ensure_chat_settings(&state.db, msg.chat.id.0).await?;

            if let Some(text) = msg.text() {
                if text.starts_with("/start") {
                    bot.send_message(msg.chat.id, "Woxel подключен. Панель ниже:")
                        .reply_markup(group_panel_keyboard(&state.bot_name, msg.chat.id.0))
                        .await?;
                }
            }
            let _ = state
                .moderation
                .moderate_message(&bot, &state.db, &msg)
                .await;
        }
        ChatKind::Private(_) => {
            if let Some(text) = msg.text() {
                if text.starts_with("/start") || text.starts_with("/help") {
                    bot.send_message(
                        msg.chat.id,
                        "Добро пожаловать в Woxel. Управление через inline-кнопки.",
                    )
                    .reply_markup(private_main_menu())
                    .parse_mode(ParseMode::Html)
                    .await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn on_callback(bot: Bot, q: CallbackQuery, state: Arc<AppState>) -> anyhow::Result<()> {
    let user_id = q.from.id.0 as i64;
    if !state
        .limiter
        .allow(user_id, state.cfg.rate_limit_per_sec)
        .await
    {
        bot.answer_callback_query(q.id)
            .text("Слишком часто. Попробуйте позже.")
            .await?;
        return Ok(());
    }

    let data = q.data.unwrap_or_default();
    if let Some(msg) = q.message {
        if data.starts_with("check_rights:") {
            let chat_id = data
                .split(':')
                .nth(1)
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(msg.chat().id.0);
            let allowed =
                is_admin_or_superadmin(&bot, &state.cfg, ChatId(chat_id), q.from.id).await;
            bot.answer_callback_query(q.id)
                .text(if allowed {
                    "✅ Админ-доступ подтверждён"
                } else {
                    "❌ Нужны права админа"
                })
                .await?;
            return Ok(());
        }

        if data == "help" {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                "Woxel: все настройки через inline-панель в ЛС.",
            )
            .reply_markup(private_main_menu())
            .await?;
        } else if data.starts_with("rules:") {
            bot.answer_callback_query(q.id)
                .text("📜 Правила: настройка в панели ЛС.")
                .await?;
        } else if data.starts_with("menu:") {
            bot.edit_message_text(
                msg.chat().id,
                msg.id(),
                format!("Раздел: {}", data.trim_start_matches("menu:")),
            )
            .reply_markup(private_main_menu())
            .await?;
        } else if data.starts_with("verify:") {
            let parts: Vec<&str> = data.split(':').collect();
            if parts.len() == 3 {
                let target_user_id = parts[2].parse::<u64>().unwrap_or_default();
                if q.from.id.0 == target_user_id {
                    bot.answer_callback_query(q.id)
                        .text("✅ Верификация пройдена")
                        .await?;
                } else {
                    bot.answer_callback_query(q.id)
                        .text("Эта кнопка не для вас")
                        .await?;
                }
            }
        }
    }
    Ok(())
}

pub async fn on_chat_member(
    bot: Bot,
    upd: ChatMemberUpdated,
    state: Arc<AppState>,
) -> anyhow::Result<()> {
    let chat_id = upd.chat.id;
    if matches!(
        upd.new_chat_member.kind,
        ChatMemberKind::Member(_) | ChatMemberKind::Restricted(_)
    ) {
        let user = &upd.new_chat_member.user;
        upsert_user(&state.db, user.id.0 as i64, user.username.as_deref()).await?;
        let _ = bot
            .restrict_chat_member(chat_id, user.id, ChatPermissions::empty())
            .await;
        let kb = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "✅ Я не бот",
            format!("verify:{}:{}", chat_id.0, user.id.0),
        )]]);
        let _ = bot
            .send_message(
                chat_id,
                format!("👋 {}, нажмите кнопку для верификации", user.full_name()),
            )
            .reply_markup(kb)
            .await;
    }
    Ok(())
}
