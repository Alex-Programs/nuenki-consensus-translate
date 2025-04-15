use get_source::get_appropriate_sources;
use serde::Serialize;
use Languages::TargetLanguage;

mod deepl;
mod get_source;
mod openrouter;

type ModelName = &'static str;

pub enum TranslationSource {
    Openrouter(ModelName),
    Deepl,
}

// rough flow:
// - take in sentence
// - get what we're going to use to translate and eval
// translate the different sentences
// eval. This ranks the sentences and produces a new, synthesised one with the best aspects of them all
// return.

#[derive(Serialize)]
pub struct TranslationResponse {
    translations: Vec<TranslationResponseItem>,
}

#[derive(Serialize)]
pub struct TranslationResponseItem {
    model: String,
    combined: bool,
    text: String,
    score: u32,
}

pub enum Formality {
    LessFormal,
    NormalFormality,
    MoreFormal,
}

pub fn consensus_translate(
    language: TargetLanguage,
    formality: Formality,
) -> Result<TranslationResponse, String> {
    let translation_methods = get_appropriate_sources(language);
}
