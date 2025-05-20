use crate::{languages::Language, TranslationSource};

const GPT4O: &'static str = "openai/gpt-4o-2024-11-20";
const GPT41: &'static str = "openai/gpt-4.1";
const GEMINI_FLASH2_5: &'static str = "google/gemini-2.5-flash-preview";
const LLAMA33_70B: &'static str = "meta-llama/llama-3.3-70b-instruct";
const LLAMA4MAV: &'static str = "meta-llama/llama-4-maverick";
const DEEPSEEKV3: &'static str = "deepseek/deepseek-chat-v3-0324";
const SONNET37: &'static str = "anthropic/claude-3.7-sonnet";
const GEMMA3_27B: &'static str = "google/gemma-3-27b-it";
const GROK3: &'static str = "x-ai/grok-3-beta";

pub struct SourceResponse {
    pub translate_sources: Vec<TranslationSource>,
    pub eval_source: TranslationSource,
}

pub fn get_appropriate_sources(target_lang: Language) -> SourceResponse {
    match target_lang {
        Language::Chinese | Language::ChineseTraditional => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(GPT41),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
                TranslationSource::Openrouter(DEEPSEEKV3),
                TranslationSource::Openrouter(GROK3),
            ],
            eval_source: TranslationSource::Openrouter(GPT4O),
        },
        Language::Esperanto => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT41),
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(GROK3),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::French => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(LLAMA33_70B),
                TranslationSource::Openrouter(LLAMA4MAV),
                TranslationSource::Openrouter(DEEPSEEKV3),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
                TranslationSource::Openrouter(GPT41),
                TranslationSource::Openrouter(GPT4O),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::German => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(GEMMA3_27B),
                TranslationSource::Openrouter(GROK3),
                TranslationSource::Openrouter(LLAMA4MAV),
                TranslationSource::Openrouter(DEEPSEEKV3),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Hungarian => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(SONNET37),
                TranslationSource::Openrouter(GROK3),
                TranslationSource::Openrouter(LLAMA4MAV),
                TranslationSource::Openrouter(DEEPSEEKV3),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Italian => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT41),
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(GEMMA3_27B),
                TranslationSource::Openrouter(LLAMA4MAV),
                TranslationSource::Openrouter(DEEPSEEKV3),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Japanese => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(SONNET37),
                TranslationSource::Openrouter(GROK3),
                TranslationSource::Openrouter(LLAMA4MAV),
                TranslationSource::Openrouter(DEEPSEEKV3),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Korean => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT41),
                TranslationSource::Openrouter(SONNET37),
                TranslationSource::Openrouter(GEMMA3_27B),
                TranslationSource::Openrouter(GROK3),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Spanish => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(SONNET37),
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(LLAMA4MAV),
                TranslationSource::Openrouter(DEEPSEEKV3),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Swedish => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(LLAMA33_70B),
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(GPT41),
                TranslationSource::Openrouter(GROK3),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Ukrainian => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT41),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(GROK3),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Vietnamese => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT41),
                TranslationSource::Openrouter(GEMMA3_27B),
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(GROK3),
                TranslationSource::Openrouter(DEEPSEEKV3),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Welsh | Language::Thai | Language::Klingon => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GPT41),
                TranslationSource::Openrouter(SONNET37),
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(GROK3),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
        Language::Unknown | _ => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(GEMMA3_27B),
                TranslationSource::Openrouter(GPT4O),
                TranslationSource::Openrouter(LLAMA4MAV),
                TranslationSource::Openrouter(DEEPSEEKV3),
                TranslationSource::Openrouter(GEMINI_FLASH2_5),
            ],
            eval_source: TranslationSource::Openrouter(GPT41),
        },
    }
}
