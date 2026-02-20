mod bot;
mod config;
mod db;
mod i18n;
mod moderation;
mod utils;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(cfg.rust_log.clone()))
        .with_target(false)
        .compact()
        .init();

    let db = db::init_pool(&cfg.database_url).await?;
    let bot = teloxide::Bot::new(cfg.bot_token.clone());

    bot::run(bot, cfg, db).await
}
