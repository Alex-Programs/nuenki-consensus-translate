use crate::Formality;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct TranslateRequest {
    text: Vec<String>,
    target_lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_lang: Option<String>,
    formality: String,
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
    client: Client,
}

impl DeepLClient {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn translate(
        &self,
        text: &str,
        target_lang: &str,
        source_lang: Option<&str>,
        formality: Formality,
    ) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/translate", self.base_url);
        let formality_str = match formality {
            Formality::LessFormal => "less".to_string(),
            Formality::NormalFormality => "default".to_string(),
            Formality::MoreFormal => "more".to_string(),
        };
        let request_body = TranslateRequest {
            text: vec![text.to_string()],
            target_lang: target_lang.to_string(),
            source_lang: source_lang.map(|s| s.to_string()),
            formality: formality_str,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("DeepL-Auth-Key {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let translate_response: TranslateResponse = response.json().await?;
        if translate_response.translations.is_empty() {
            return Err("No translations returned from DeepL API".into());
        }

        Ok(translate_response.translations[0].text.clone())
    }
}
