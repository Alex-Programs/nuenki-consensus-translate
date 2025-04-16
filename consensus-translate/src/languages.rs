use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
pub enum Language {
    Arabic,
    ArabicStandard,
    Bulgarian,
    Chinese,
    ChineseTraditional,
    Croatian,
    Czech,
    Danish,
    Dutch,
    Esperanto,
    Estonian,
    Finnish,
    French,
    German,
    Greek,
    Hebrew,
    Hindi,
    Hungarian,
    Indonesian,
    Italian,
    Japanese,
    Korean,
    LatinClassical,
    Latvian,
    Lithuanian,
    Norwegian,
    Persian,
    Polish,
    PortugueseBrazil,
    PortuguesePortugal,
    Romanian,
    Russian,
    Slovakian,
    Slovenian,
    Spanish,
    Swedish,
    Turkish,
    Ukrainian,
    Vietnamese,
    English,
    Unknown,
}

impl Language {
    pub fn to_llm_format(&self) -> String {
        match self {
            Language::Arabic => "Arabic".to_string(),
            Language::ArabicStandard => "Arabic (Standard)".to_string(),
            Language::Bulgarian => "Bulgarian".to_string(),
            Language::Chinese => "Chinese (Simplified)".to_string(),
            Language::ChineseTraditional => "Chinese (Traditional)".to_string(),
            Language::Croatian => "Croatian".to_string(),
            Language::Czech => "Czech".to_string(),
            Language::Danish => "Danish".to_string(),
            Language::Dutch => "Dutch".to_string(),
            Language::Esperanto => "Esperanto".to_string(),
            Language::Estonian => "Estonian".to_string(),
            Language::Finnish => "Finnish".to_string(),
            Language::French => "French".to_string(),
            Language::German => "German".to_string(),
            Language::Greek => "Greek".to_string(),
            Language::Hebrew => "Hebrew".to_string(),
            Language::Hindi => "Hindi".to_string(),
            Language::Hungarian => "Hungarian".to_string(),
            Language::Indonesian => "Indonesian".to_string(),
            Language::Italian => "Italian".to_string(),
            Language::Japanese => "Japanese".to_string(),
            Language::Korean => "Korean".to_string(),
            Language::LatinClassical => "Latin (Classical)".to_string(),
            Language::Latvian => "Latvian".to_string(),
            Language::Lithuanian => "Lithuanian".to_string(),
            Language::Norwegian => "Norwegian".to_string(),
            Language::Persian => "Persian".to_string(),
            Language::Polish => "Polish".to_string(),
            Language::PortugueseBrazil => "Portuguese (Brazil)".to_string(),
            Language::PortuguesePortugal => "Portuguese (Portugal)".to_string(),
            Language::Romanian => "Romanian".to_string(),
            Language::Russian => "Russian".to_string(),
            Language::Slovakian => "Slovak".to_string(),
            Language::Slovenian => "Slovenian".to_string(),
            Language::Spanish => "Spanish".to_string(),
            Language::Swedish => "Swedish".to_string(),
            Language::Turkish => "Turkish".to_string(),
            Language::Ukrainian => "Ukrainian".to_string(),
            Language::Vietnamese => "Vietnamese".to_string(),
            Language::English => "English".to_string(),
            Language::Unknown => "an unspecified language".to_string(),
        }
    }
}
