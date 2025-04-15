use reqwest::Client;
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
    usage: Usage,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

#[derive(Deserialize)]
struct Usage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
    total_cost: f64, // Cost provided by OpenRouter
}

pub struct OpenRouterClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl OpenRouterClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            client: Client::new(),
        }
    }

    pub async fn complete(
        &self,
        system_prompt: &str,
        main_prompt: &str,
        model: &str,
        temperature: f32,
    ) -> Result<(String, f64), Box<dyn Error>> {
        let url = format!("{}/chat/completions", self.base_url);
        let request_body = ChatRequest {
            model: model.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: main_prompt.to_string(),
                },
            ],
            temperature,
        };
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;
        let chat_response: ChatResponse = response.json().await?;
        if chat_response.choices.is_empty() {
            return Err("No choices returned from OpenRouter API".into());
        }
        Ok((
            chat_response.choices[0].message.content.clone(),
            chat_response.usage.total_cost,
        ))
    }
}
