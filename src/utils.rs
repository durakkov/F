use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use url::Url;

pub fn extract_domains(text: &str) -> Vec<String> {
    let mut domains = Vec::new();
    for token in text.split_whitespace() {
        if let Ok(url) = Url::parse(token) {
            if let Some(host) = url.host_str() {
                domains.push(host.to_ascii_lowercase());
            }
        }
    }
    domains
}

pub async fn raw_api_call(token: &str, method: &str, payload: Value) -> Result<Value> {
    let url = format!("https://api.telegram.org/bot{token}/{method}");
    let client = Client::new();
    let resp = client.post(url).json(&payload).send().await?;
    let status = resp.status();
    let body: Value = resp.json().await?;
    if !status.is_success() || body.get("ok").and_then(Value::as_bool) != Some(true) {
        anyhow::bail!("telegram api error on {method}: {}", body);
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_domains() {
        let found = extract_domains("hi https://example.com/test and https://sub.foo.bar/x");
        assert_eq!(found, vec!["example.com", "sub.foo.bar"]);
    }
}
