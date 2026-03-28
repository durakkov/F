# Woxel

Woxel — production-ready Telegram moderation bot for groups/supergroups/forums with **inline-only UX** (buttons + callback queries). Slash commands are only emergency entry points (`/start`, optional `/help`).

## Features (MVP)
- Inline admin panel in private chat.
- Group mini panel with quick status/rules/open-panel buttons.
- Forbidden words/phrases + regex filters.
- Link/domain policies: deny-all, deny-list, allow-list.
- Anti-flood with time window and threshold.
- Join verification button (`✅ Я не бот`) and restrictions.
- Warn system with escalation hooks.
- Moderation action logging to DB.
- Polling and webhook modes via env config.
- SQLite default, Postgres supported through `DATABASE_URL`.
- Docker + systemd + CI included.

## Quick start
### 1) BotFather
1. Create bot in `@BotFather` (`/newbot`) and copy token.
2. Disable privacy mode if moderation should inspect all messages in groups.

### 2) Configure environment
```bash
cp .env.example .env
# edit BOT_TOKEN and values
```

### 3) Run locally (polling)
```bash
cargo run
```

## Telegram setup
1. Add bot to group/supergroup.
2. Grant admin rights:
   - Delete messages
   - Restrict members
   - Ban users
   - (Optional) Pin messages
3. In group send `/start` once to publish Woxel inline service message.
4. Click `🛠 Открыть панель` (deep-link to private bot chat), then configure via buttons.

## Webhook mode
Set:
- `MODE=webhook`
- `WEBHOOK_URL=https://example.com/tg/webhook`
- `WEBHOOK_BIND=0.0.0.0:8080`
- `WEBHOOK_PATH=/tg/webhook`

For HTTPS use Nginx + Certbot:
- terminate TLS in nginx,
- proxy `/tg/webhook` to `127.0.0.1:8080`.

## Docker deploy
```bash
docker compose up -d --build
```

## systemd deploy
1. Build binary and copy to `/usr/local/bin/woxel`.
2. Put repo to `/opt/woxel`, copy `.env`.
3. Install unit:
```bash
sudo cp deploy/woxel.service /etc/systemd/system/woxel.service
sudo systemctl daemon-reload
sudo systemctl enable --now woxel
```

## Database and migrations
Migrations run automatically at startup via sqlx migrator.
Manual run example:
```bash
sqlx migrate run
```

## Troubleshooting
- Bot cannot delete messages: verify bot admin rights in group.
- No updates in webhook mode: verify public HTTPS URL and reverse proxy route.
- DB connection errors: validate `DATABASE_URL`.
- Not seeing all messages: disable BotFather privacy mode.

## Assumptions
- Admin panel is private-chat first; group panel is intentionally minimal.
- Verification flow restricts new member and asks to press inline button; timeout enforcement can be extended from current base behavior.
