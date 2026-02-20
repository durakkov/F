CREATE TABLE IF NOT EXISTS chats (
    id INTEGER PRIMARY KEY,
    tg_chat_id BIGINT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_settings (
    tg_chat_id BIGINT PRIMARY KEY,
    language TEXT NOT NULL DEFAULT 'ru',
    moderation_enabled BOOLEAN NOT NULL DEFAULT true,
    links_mode TEXT NOT NULL DEFAULT 'deny_all',
    flood_window_sec INT NOT NULL DEFAULT 5,
    flood_max_msgs INT NOT NULL DEFAULT 6,
    warn_threshold INT NOT NULL DEFAULT 3,
    default_sanction TEXT NOT NULL DEFAULT 'warn',
    mute_minutes INT NOT NULL DEFAULT 10,
    raid_window_sec INT NOT NULL DEFAULT 60,
    raid_max_joins INT NOT NULL DEFAULT 10,
    lockdown_minutes INT NOT NULL DEFAULT 5,
    verification_enabled BOOLEAN NOT NULL DEFAULT false,
    verification_timeout_min INT NOT NULL DEFAULT 10,
    log_chat_id BIGINT NULL,
    log_thread_id BIGINT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    tg_user_id BIGINT NOT NULL UNIQUE,
    username TEXT NULL,
    first_seen TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_users (
    tg_chat_id BIGINT NOT NULL,
    tg_user_id BIGINT NOT NULL,
    warn_count INT NOT NULL DEFAULT 0,
    last_violation_at TIMESTAMP NULL,
    whitelisted BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (tg_chat_id, tg_user_id)
);

CREATE TABLE IF NOT EXISTS filters (
    id INTEGER PRIMARY KEY,
    tg_chat_id BIGINT NOT NULL,
    kind TEXT NOT NULL,
    pattern TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE IF NOT EXISTS domains (
    id INTEGER PRIMARY KEY,
    tg_chat_id BIGINT NOT NULL,
    list_type TEXT NOT NULL,
    domain TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE IF NOT EXISTS mod_actions (
    id INTEGER PRIMARY KEY,
    tg_chat_id BIGINT NOT NULL,
    tg_user_id BIGINT NOT NULL,
    action TEXT NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    actor_user_id BIGINT NULL
);
