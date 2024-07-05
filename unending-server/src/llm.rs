const GPT4ALL_URL: &str = "127.0.0.1:4891";
const GPT4ALL_MODEL: &str = "Llama 3 Instruct";
const GPT4ALL_TIMEOUT: u64 = 90;
const GPT4ALL_ATTEMPTS: usize = 3;

use crate::log;

// TODO: make tokens an Option and handle None
fn openai_body(message: &str, tokens: usize) -> String {
    let sanitized = message.replace('"', "'"); // TODO: more
    format!(
        r#"
        {{
            "model": "{GPT4ALL_MODEL}",
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
        .timeout(std::time::Duration::from_secs(GPT4ALL_TIMEOUT))
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

pub fn gpt4all_chat_force(message: &str, tokens: usize) -> String {
    for _ in 0..GPT4ALL_ATTEMPTS {
        if let Some(response) = gpt4all_chat(message, tokens) {
            return response;
        }
    }
    panic!(
            "{}\n{}\n{}",
            "Failed to get response from gpt4all.",
            format!("Message: {message}"),
            format!("Tokens: {tokens}"),
    ); // TODO: could figure out some sort of restart, or general better fail state
}