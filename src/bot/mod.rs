pub mod handlers;
pub mod middleware;
pub mod ui;

use std::sync::Arc;

use anyhow::Result;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::prelude::*;
use tracing::info;

use crate::bot::handlers::{on_callback, on_chat_member, on_message, AppState};
use crate::config::{Config, RunMode};
use crate::db::queries::DbPool;
use crate::moderation::ModerationEngine;

pub async fn run(bot: Bot, cfg: Config, db: DbPool) -> Result<()> {
    let me = bot.get_me().await?;
    let state = Arc::new(AppState {
        cfg: cfg.clone(),
        db,
        limiter: Arc::new(crate::bot::middleware::CallbackRateLimit::default()),
        moderation: Arc::new(ModerationEngine::new()),
        bot_name: me.user.username.unwrap_or_else(|| "WoxelBot".into()),
    });

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(on_message))
        .branch(Update::filter_callback_query().endpoint(on_callback))
        .branch(Update::filter_chat_member().endpoint(on_chat_member));

    let mut dispatcher = Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![state])
        .default_handler(|upd| async move {
            info!("unhandled update: {upd:?}");
        })
        .error_handler(LoggingErrorHandler::with_custom_text("dispatcher error"))
        .build();

    match cfg.mode {
        RunMode::Polling => dispatcher.dispatch().await,
        RunMode::Webhook => {
            let wh = cfg.webhook.expect("validated webhook config");
            let listener = teloxide::update_listeners::webhooks::axum(
                bot,
                teloxide::types::Url::parse(&wh.url)?,
            )
            .await?
            .path(wh.path)
            .bind(wh.bind);
            dispatcher
                .dispatch_with_listener(
                    listener,
                    LoggingErrorHandler::with_custom_text("webhook listener"),
                )
                .await;
        }
    }

    Ok(())
}
