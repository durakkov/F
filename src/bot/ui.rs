use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn group_panel_keyboard(bot_username: &str, chat_id: i64) -> InlineKeyboardMarkup {
    let deep_link = format!("https://t.me/{bot_username}?start=panel_{chat_id}");
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::url(
            "🛠 Открыть панель",
            deep_link.parse().unwrap(),
        )],
        vec![
            InlineKeyboardButton::callback("✅ Проверить права", format!("check_rights:{chat_id}")),
            InlineKeyboardButton::callback("📜 Правила", format!("rules:{chat_id}")),
        ],
        vec![InlineKeyboardButton::callback(
            "ℹ️ Справка",
            "help".to_string(),
        )],
    ])
}

pub fn private_main_menu() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Выбрать чат", "menu:chats")],
        vec![InlineKeyboardButton::callback(
            "Настройки модерации",
            "menu:mod",
        )],
        vec![InlineKeyboardButton::callback("Фильтры", "menu:filters")],
        vec![InlineKeyboardButton::callback("Ссылки", "menu:links")],
        vec![InlineKeyboardButton::callback("Антифлуд", "menu:flood")],
        vec![InlineKeyboardButton::callback(
            "Антирейд/локдаун",
            "menu:raid",
        )],
        vec![InlineKeyboardButton::callback(
            "Приветствие/верификация",
            "menu:welcome",
        )],
        vec![InlineKeyboardButton::callback("Логи", "menu:logs")],
        vec![InlineKeyboardButton::callback(
            "Dry-run сообщения",
            "menu:dryrun",
        )],
        vec![InlineKeyboardButton::callback(
            "Статус/диагностика",
            "menu:status",
        )],
    ])
}
