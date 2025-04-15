use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct TranslateRequest {
    text: Vec<String>,
    target_lang: String,
    source_lang: Option<String>,
    formality: Option<String>,
}

#[derive(Deserialize)]
struct TranslateResponse {
    translations: Vec<Translation>,
}

#[derive(Deserialize)]
struct Translation {
    text: String,
}

pub struct DeepLClient {
    api_key: String,
    base_url: String,
}

impl DeepLClient {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub fn translate(
        &self,
        text: &str,
        target_lang: &str,
        source_lang: Option<&str>,
        formality: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/translate", self.base_url);
        let client = Client::new();
        let request_body = TranslateRequest {
            text: vec![text.to_string()],
            target_lang: target_lang.to_string(),
            source_lang: source_lang.map(|s| s.to_string()),
            formality: formality.map(|f| f.to_string()),
        };

        let response = client
            .post(&url)
            .header("Authorization", format!("DeepL-Auth-Key {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()?;

        let translate_response: TranslateResponse = response.json()?;
        if translate_response.translations.is_empty() {
            return Err("No translations returned in the response".into());
        }

        Ok(translate_response.translations[0].text.clone())
    }
}
