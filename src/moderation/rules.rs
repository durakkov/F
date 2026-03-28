use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use regex::Regex;

use crate::db::models::{ChatSettings, DomainRule, Filter};
use crate::utils::extract_domains;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationKind {
    Filtered,
    LinkDenied,
    Flood,
}

#[derive(Debug, Clone)]
pub struct FloodTracker {
    // (chat,user) -> timestamps
    events: HashMap<(i64, i64), VecDeque<Instant>>,
}

impl FloodTracker {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn hit(&mut self, chat_id: i64, user_id: i64, window_sec: i32, max_msgs: i32) -> bool {
        let key = (chat_id, user_id);
        let now = Instant::now();
        let q = self.events.entry(key).or_default();
        q.push_back(now);
        while let Some(front) = q.front() {
            if now.duration_since(*front) > Duration::from_secs(window_sec as u64) {
                q.pop_front();
            } else {
                break;
            }
        }
        q.len() as i32 > max_msgs
    }
}

pub fn check_filters(text: &str, filters: &[Filter]) -> bool {
    let lower = text.to_ascii_lowercase();
    filters.iter().any(|f| {
        if !f.enabled {
            return false;
        }
        match f.kind.as_str() {
            "text" => lower.contains(&f.pattern.to_ascii_lowercase()),
            "regex" => Regex::new(&f.pattern)
                .map(|r| r.is_match(text))
                .unwrap_or(false),
            _ => false,
        }
    })
}

pub fn check_links(text: &str, settings: &ChatSettings, rules: &[DomainRule]) -> bool {
    let domains = extract_domains(text);
    if domains.is_empty() {
        return false;
    }
    match settings.links_mode.as_str() {
        "deny_all" => true,
        "deny_list" => {
            let denied: Vec<_> = rules
                .iter()
                .filter(|r| r.enabled && r.list_type == "deny")
                .map(|r| r.domain.to_ascii_lowercase())
                .collect();
            domains
                .iter()
                .any(|d| denied.iter().any(|bad| d.ends_with(bad)))
        }
        "allow_list" => {
            let allowed: Vec<_> = rules
                .iter()
                .filter(|r| r.enabled && r.list_type == "allow")
                .map(|r| r.domain.to_ascii_lowercase())
                .collect();
            domains
                .iter()
                .any(|d| !allowed.iter().any(|ok| d.ends_with(ok)))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(mode: &str) -> ChatSettings {
        ChatSettings {
            tg_chat_id: 1,
            language: "ru".into(),
            moderation_enabled: true,
            links_mode: mode.into(),
            flood_window_sec: 5,
            flood_max_msgs: 3,
            warn_threshold: 3,
            default_sanction: "warn".into(),
            mute_minutes: 10,
            raid_window_sec: 60,
            raid_max_joins: 10,
            lockdown_minutes: 5,
            verification_enabled: false,
            verification_timeout_min: 10,
            log_chat_id: None,
            log_thread_id: None,
        }
    }

    #[test]
    fn text_and_regex_filter() {
        let filters = vec![
            Filter {
                id: 1,
                tg_chat_id: 1,
                kind: "text".into(),
                pattern: "spam".into(),
                enabled: true,
            },
            Filter {
                id: 2,
                tg_chat_id: 1,
                kind: "regex".into(),
                pattern: "buy\\s+now".into(),
                enabled: true,
            },
        ];
        assert!(check_filters("This is SPAM", &filters));
        assert!(check_filters("please buy now", &filters));
        assert!(!check_filters("normal message", &filters));
    }

    #[test]
    fn flood_window_works() {
        let mut tr = FloodTracker::new();
        assert!(!tr.hit(1, 1, 5, 3));
        assert!(!tr.hit(1, 1, 5, 3));
        assert!(!tr.hit(1, 1, 5, 3));
        assert!(tr.hit(1, 1, 5, 3));
    }

    #[test]
    fn domain_modes_work() {
        let rules = vec![
            DomainRule {
                id: 1,
                tg_chat_id: 1,
                list_type: "deny".into(),
                domain: "bad.com".into(),
                enabled: true,
            },
            DomainRule {
                id: 2,
                tg_chat_id: 1,
                list_type: "allow".into(),
                domain: "good.com".into(),
                enabled: true,
            },
        ];
        assert!(check_links(
            "https://a.bad.com",
            &settings("deny_list"),
            &rules
        ));
        assert!(check_links(
            "https://evil.com",
            &settings("allow_list"),
            &rules
        ));
        assert!(!check_links(
            "https://a.good.com",
            &settings("allow_list"),
            &rules
        ));
    }
}
