#[derive(Debug, Clone, Copy)]
pub enum Lang {
    Ru,
    En,
}

pub fn tr(lang: Lang, key: &str) -> &'static str {
    match (lang, key) {
        (Lang::Ru, "panel") => "Панель Woxel",
        (Lang::Ru, "open_panel") => "🛠 Открыть панель",
        (Lang::Ru, "rules") => "📜 Правила",
        (Lang::Ru, "help") => "ℹ️ Справка",
        (Lang::En, "panel") => "Woxel Panel",
        (Lang::En, "open_panel") => "🛠 Open panel",
        (Lang::En, "rules") => "📜 Rules",
        (Lang::En, "help") => "ℹ️ Help",
        _ => "",
    }
}
