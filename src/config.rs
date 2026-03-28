use std::{env, net::SocketAddr, str::FromStr};

use anyhow::{anyhow, bail, Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    Polling,
    Webhook,
}

impl FromStr for RunMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "polling" => Ok(Self::Polling),
            "webhook" => Ok(Self::Webhook),
            _ => bail!("MODE must be 'polling' or 'webhook'"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub bot_token: String,
    pub mode: RunMode,
    pub database_url: String,
    pub admin_ids: Vec<i64>,
    pub rust_log: String,
    pub rate_limit_per_sec: u32,
    pub webhook: Option<WebhookConfig>,
}

#[derive(Debug, Clone)]
pub struct WebhookConfig {
    pub url: String,
    pub bind: SocketAddr,
    pub path: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        let bot_token = required("BOT_TOKEN")?;
        let mode = required("MODE")?.parse::<RunMode>()?;
        let database_url = required("DATABASE_URL")?;
        let admin_ids = env::var("ADMIN_IDS")
            .unwrap_or_default()
            .split(',')
            .filter(|v| !v.trim().is_empty())
            .map(|v| v.trim().parse::<i64>().context("invalid ADMIN_IDS entry"))
            .collect::<Result<Vec<_>>>()?;
        let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        let rate_limit_per_sec = env::var("RATE_LIMIT_PER_SEC")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u32>()
            .context("RATE_LIMIT_PER_SEC must be integer")?;

        let webhook = if mode == RunMode::Webhook {
            let url = required("WEBHOOK_URL")?;
            let bind = required("WEBHOOK_BIND")?
                .parse::<SocketAddr>()
                .context("WEBHOOK_BIND should be host:port")?;
            let path = env::var("WEBHOOK_PATH").unwrap_or_else(|_| "/tg/webhook".to_string());
            Some(WebhookConfig { url, bind, path })
        } else {
            None
        };

        let cfg = Self {
            bot_token,
            mode,
            database_url,
            admin_ids,
            rust_log,
            rate_limit_per_sec,
            webhook,
        };
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn validate(&self) -> Result<()> {
        if self.bot_token.trim().is_empty() {
            bail!("BOT_TOKEN cannot be empty");
        }
        if !(self.database_url.starts_with("sqlite://")
            || self.database_url.starts_with("postgres://"))
        {
            bail!("DATABASE_URL must start with sqlite:// or postgres://");
        }
        if self.rate_limit_per_sec == 0 {
            bail!("RATE_LIMIT_PER_SEC must be > 0");
        }
        if self.mode == RunMode::Webhook {
            let wh = self
                .webhook
                .as_ref()
                .ok_or_else(|| anyhow!("webhook config required"))?;
            if !wh.url.starts_with("https://") {
                bail!("WEBHOOK_URL must be https");
            }
            if !wh.path.starts_with('/') {
                bail!("WEBHOOK_PATH must start with '/'");
            }
        }
        Ok(())
    }
}

fn required(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("missing required env var: {key}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rejects_bad_db() {
        let cfg = Config {
            bot_token: "x".into(),
            mode: RunMode::Polling,
            database_url: "mysql://bad".into(),
            admin_ids: vec![],
            rust_log: "info".into(),
            rate_limit_per_sec: 1,
            webhook: None,
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn validate_webhook_requires_https() {
        let cfg = Config {
            bot_token: "x".into(),
            mode: RunMode::Webhook,
            database_url: "sqlite://woxel.db".into(),
            admin_ids: vec![],
            rust_log: "info".into(),
            rate_limit_per_sec: 1,
            webhook: Some(WebhookConfig {
                url: "http://example.com".into(),
                bind: "0.0.0.0:8080".parse().unwrap(),
                path: "/x".into(),
            }),
        };
        assert!(cfg.validate().is_err());
    }
}
