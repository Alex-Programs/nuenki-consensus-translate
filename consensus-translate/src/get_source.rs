use crate::TranslationSource;
use Languages::TargetLanguage;

const gpt4o: &'static str = "openai/gpt-4o-2024-11-20";
const gpt41: &'static str = "openai/openai/gpt-4.1";
const geminiflash2: &'static str = "google/gemini-2.0-flash-001";
const llama3370b: &'static str = "meta-llama/llama-3.3-70b-instruct";
const sonnet35: &'static str = "anthropic/claude-3.5-sonnet";
const gemma3_27b: &'static str = "google/gemma-3-27b-it";

pub struct SourceResponse {
    translate_sources: Vec<TranslationSource>,
    eval_source: TranslationSource,
}

pub fn get_appropriate_sources(target_lang: TargetLanguage) -> SourceResponse {
    match target_lang {
        TargetLanguage::Chinese | TargetLanguage::ChineseTraditional => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt4o),
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(geminiflash2),
            ],
            eval_source: TranslationSource::Openrouter(gpt4o),
        },
        TargetLanguage::Esperanto => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(sonnet35),
                TranslationSource::Openrouter(gpt4o),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::French => SourceResponse {
            translate_sources: vec![
                TranslationSource::Deepl,
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(gpt4o),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::German => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(gpt4o),
                TranslationSource::Openrouter(gemma3_27b),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::Hungarian => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(gpt4o),
                TranslationSource::Openrouter(sonnet35),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::Italian => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(gpt4o),
                TranslationSource::Openrouter(gemma3_27b),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::Japanese => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(gpt4o),
                TranslationSource::Deepl,
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::Korean => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(sonnet35),
                TranslationSource::Openrouter(gemma3_27b),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::Spanish => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(sonnet35),
                TranslationSource::Deepl,
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::Swedish => SourceResponse {
            translate_sources: vec![
                TranslationSource::Deepl,
                TranslationSource::Openrouter(gpt4o),
                TranslationSource::Openrouter(gpt41),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::Ukrainian => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(geminiflash2),
                TranslationSource::Openrouter(gpt4o),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::Vietnamese => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(gemma3_27b),
                TranslationSource::Openrouter(gpt4o),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        _ => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(gemma3_27b),
                TranslationSource::Openrouter(gpt4o),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
    }
}
