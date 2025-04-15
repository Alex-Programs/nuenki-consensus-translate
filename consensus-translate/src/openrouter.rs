use Languages::TargetLanguage;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

pub struct OpenRouterClient {
    api_key: String,
    base_url: String,
}

impl OpenRouterClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
        }
    }

    pub fn complete(
        &self,
        input_text: &str,
        model: &str,
        temperature: f32,
    ) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/chat/completions", self.base_url);
        let client = Client::new();
        let request_body = ChatRequest {
            model: model.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: input_text.to_string(),
            }],
            temperature,
        };

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()?;

        let chat_response: ChatResponse = response.json()?;
        if chat_response.choices.is_empty() {
            return Err("No choices returned in the response".into());
        }

        Ok(chat_response.choices[0].message.content.clone())
    }
}
