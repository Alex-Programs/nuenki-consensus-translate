use crate::TranslationSource;
use Languages::TargetLanguage;

const gpt4o: &'static str = "openai/gpt-4o-2024-11-20";
const gpt41: &'static str = "openai/openai/gpt-4.1";
const geminiflash2: &'static str = "google/gemini-2.0-flash-001";
const llama3370b: &'static str = "meta-llama/llama-3.3-70b-instruct";
const sonnet35: &'static str = "anthropic/claude-3.5-sonnet";
const gemma3_27b: &'static str = "google/gemma-3-27b-it";

pub struct SourceResponse {
    pub translate_sources: Vec<TranslationSource>,
    pub eval_source: TranslationSource,
}

// I realised halfway through making this that
// the deepl TOS really doesn't like "derivative" products like this :/
// so we're making it default-disabled

pub fn get_appropriate_sources(
    target_lang: TargetLanguage,
    is_deepl_disabled: bool,
) -> SourceResponse {
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
        TargetLanguage::French => match is_deepl_disabled {
            true => SourceResponse {
                translate_sources: vec![
                    TranslationSource::Openrouter(llama3370b),
                    TranslationSource::Openrouter(gpt41),
                    TranslationSource::Openrouter(gpt4o),
                ],
                eval_source: TranslationSource::Openrouter(gpt41),
            },
            false => SourceResponse {
                translate_sources: vec![
                    TranslationSource::Deepl,
                    TranslationSource::Openrouter(gpt41),
                    TranslationSource::Openrouter(gpt4o),
                ],
                eval_source: TranslationSource::Openrouter(gpt41),
            },
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
        TargetLanguage::Japanese => match is_deepl_disabled {
            true => SourceResponse {
                translate_sources: vec![
                    TranslationSource::Openrouter(gpt41),
                    TranslationSource::Openrouter(gpt4o),
                    TranslationSource::Openrouter(sonnet35),
                ],
                eval_source: TranslationSource::Openrouter(gpt41),
            },
            false => SourceResponse {
                translate_sources: vec![
                    TranslationSource::Openrouter(gpt41),
                    TranslationSource::Openrouter(gpt4o),
                    TranslationSource::Deepl,
                ],
                eval_source: TranslationSource::Openrouter(gpt41),
            },
        },
        TargetLanguage::Korean => SourceResponse {
            translate_sources: vec![
                TranslationSource::Openrouter(gpt41),
                TranslationSource::Openrouter(sonnet35),
                TranslationSource::Openrouter(gemma3_27b),
            ],
            eval_source: TranslationSource::Openrouter(gpt41),
        },
        TargetLanguage::Spanish => match is_deepl_disabled {
            true => SourceResponse {
                translate_sources: vec![
                    TranslationSource::Openrouter(gpt41),
                    TranslationSource::Openrouter(sonnet35),
                    TranslationSource::Openrouter(gpt4o),
                ],
                eval_source: TranslationSource::Openrouter(gpt41),
            },
            false => SourceResponse {
                translate_sources: vec![
                    TranslationSource::Openrouter(gpt41),
                    TranslationSource::Openrouter(sonnet35),
                    TranslationSource::Deepl,
                ],
                eval_source: TranslationSource::Openrouter(gpt41),
            },
        },
        TargetLanguage::Swedish => match is_deepl_disabled {
            true => SourceResponse {
                translate_sources: vec![
                    TranslationSource::Openrouter(llama3370b),
                    TranslationSource::Openrouter(gpt4o),
                    TranslationSource::Openrouter(gpt41),
                ],
                eval_source: TranslationSource::Openrouter(gpt41),
            },
            false => SourceResponse {
                translate_sources: vec![
                    TranslationSource::Deepl,
                    TranslationSource::Openrouter(gpt4o),
                    TranslationSource::Openrouter(gpt41),
                ],
                eval_source: TranslationSource::Openrouter(gpt41),
            },
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
        _ => {
            if target_lang.supports_deepl_n() && !is_deepl_disabled {
                SourceResponse {
                    translate_sources: vec![
                        TranslationSource::Openrouter(gpt41),
                        TranslationSource::Deepl,
                        TranslationSource::Openrouter(gpt4o),
                    ],
                    eval_source: TranslationSource::Openrouter(gpt41),
                }
            } else {
                SourceResponse {
                    translate_sources: vec![
                        TranslationSource::Openrouter(gpt41),
                        TranslationSource::Openrouter(gemma3_27b),
                        TranslationSource::Openrouter(gpt4o),
                    ],
                    eval_source: TranslationSource::Openrouter(gpt41),
                }
            }
        }
    }
}
