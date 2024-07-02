const GPT4ALL_URL: &str = "127.0.0.1:4891";
const LLM_MODEL: &str = "Llama 3 Instruct";
const ATTEMPT_TIMEOUT: u64 = 30;

use crate::log;

fn openai_body(message: &str, tokens: usize) -> String {
    let sanitized = message.replace('"', "'"); // TODO: more
    format!(
        r#"
        {{
            "model": "{LLM_MODEL}",
            "max_tokens": {tokens},
            "messages": [
                {{
                    "role": "system",
                    "content": "You are a helpful assistant."
                }},
                {{
                    "role": "user",
                    "content": "{sanitized}"
                }}
            ]
        }}
        "#
    )
}

pub fn gpt4all_chat(message: &str, tokens: usize) -> Option<String> {
    let url = "http://".to_owned() + &GPT4ALL_URL + "/v1/chat/completions";
    let client = reqwest::blocking::Client::new();
    let body = openai_body(message, tokens);
    let result = client
        .post(url)
        .body(body)
        .timeout(std::time::Duration::from_secs(ATTEMPT_TIMEOUT))
        .send();
    if result.is_err() {
        log::error(&format!("Failed LLM request: {:?}.", result));
        return None;
    }
    let json = serde_json::from_str(&result.unwrap().text().unwrap());
    if json.is_err() {
        log::error(&format!("Failed to parse LLM response: {:?}.", json));
        return None;
    }
    let value: serde_json::Value = json.unwrap();
    let choices = value.get("choices")?;
    let message = choices[0].get("message")?;
    let content = message.get("content")?;
    Some(content.to_string())
}