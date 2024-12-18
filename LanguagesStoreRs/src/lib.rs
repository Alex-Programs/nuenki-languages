use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

extern crate savefile;
use savefile::prelude::*;

#[macro_use]
extern crate savefile_derive;

#[derive(Serialize, Deserialize, Debug, EnumIter, PartialEq, Eq, Hash, Clone, Savefile)]
pub enum TargetLanguage {
    //Arabic,
    Bulgarian,
    Czech,
    Danish,
    German,
    Greek,
    Spanish,
    Estonian,
    Finnish,
    French,
    Hungarian,
    Indonesian,
    Italian,
    Japanese,
    Korean,
    Lithuanian,
    Latvian,
    Norwegian,
    Dutch,
    Polish,
    PortuguesePortugal,
    PortugueseBrazil,
    Romanian,
    Russian,
    Slovakian,
    Slovenian,
    Swedish,
    Turkish,
    Ukrainian,
    Chinese,
    Hebrew,
    Esperanto,
    Persian,
    Hawaiian,
}

impl TargetLanguage {
    pub fn from_wiktionary_language_code(code: &str) -> Option<Self> {
        match code {
            // Uncomment and add more languages as needed
            //"ar" => Some(TargetLanguage::Arabic),
            "bg" => Some(TargetLanguage::Bulgarian),
            "cs" => Some(TargetLanguage::Czech),
            "da" => Some(TargetLanguage::Danish),
            "de" => Some(TargetLanguage::German),
            "el" => Some(TargetLanguage::Greek),
            "es" => Some(TargetLanguage::Spanish),
            "et" => Some(TargetLanguage::Estonian),
            "fi" => Some(TargetLanguage::Finnish),
            "fr" => Some(TargetLanguage::French),
            "hu" => Some(TargetLanguage::Hungarian),
            "id" => Some(TargetLanguage::Indonesian),
            "it" => Some(TargetLanguage::Italian),
            "ja" => Some(TargetLanguage::Japanese),
            "ko" => Some(TargetLanguage::Korean),
            "lt" => Some(TargetLanguage::Lithuanian),
            "lv" => Some(TargetLanguage::Latvian),
            "no" => Some(TargetLanguage::Norwegian),
            "nl" => Some(TargetLanguage::Dutch),
            "pl" => Some(TargetLanguage::Polish),
            "pt-PT" => Some(TargetLanguage::PortuguesePortugal),
            "pt-BR" => Some(TargetLanguage::PortugueseBrazil),
            "ro" => Some(TargetLanguage::Romanian),
            "ru" => Some(TargetLanguage::Russian),
            "sk" => Some(TargetLanguage::Slovakian),
            "sl" => Some(TargetLanguage::Slovenian),
            "sv" => Some(TargetLanguage::Swedish),
            "tr" => Some(TargetLanguage::Turkish),
            "uk" => Some(TargetLanguage::Ukrainian),
            "zh" => Some(TargetLanguage::Chinese),
            "he" => Some(TargetLanguage::Hebrew),
            "eo" => Some(TargetLanguage::Esperanto),
            "fa" => Some(TargetLanguage::Persian),
            "haw" => Some(TargetLanguage::Hawaiian),
            _ => None, // Return None if the code doesn't match any language
        }
    }

    pub fn to_wiktionary_language_code(&self) -> String {
        match self {
            TargetLanguage::Bulgarian => "bg".to_string(),
            TargetLanguage::Czech => "cs".to_string(),
            TargetLanguage::Danish => "da".to_string(),
            TargetLanguage::German => "de".to_string(),
            TargetLanguage::Greek => "el".to_string(),
            TargetLanguage::Spanish => "es".to_string(),
            TargetLanguage::Estonian => "et".to_string(),
            TargetLanguage::Finnish => "fi".to_string(),
            TargetLanguage::French => "fr".to_string(),
            TargetLanguage::Hungarian => "hu".to_string(),
            TargetLanguage::Indonesian => "id".to_string(),
            TargetLanguage::Italian => "it".to_string(),
            TargetLanguage::Japanese => "ja".to_string(),
            TargetLanguage::Korean => "ko".to_string(),
            TargetLanguage::Lithuanian => "lt".to_string(),
            TargetLanguage::Latvian => "lv".to_string(),
            TargetLanguage::Norwegian => "no".to_string(),
            TargetLanguage::Dutch => "nl".to_string(),
            TargetLanguage::Polish => "pl".to_string(),
            TargetLanguage::PortuguesePortugal => "pt-PT".to_string(),
            TargetLanguage::PortugueseBrazil => "pt-BR".to_string(),
            TargetLanguage::Romanian => "ro".to_string(),
            TargetLanguage::Russian => "ru".to_string(),
            TargetLanguage::Slovakian => "sk".to_string(),
            TargetLanguage::Slovenian => "sl".to_string(),
            TargetLanguage::Swedish => "sv".to_string(),
            TargetLanguage::Turkish => "tr".to_string(),
            TargetLanguage::Ukrainian => "uk".to_string(),
            TargetLanguage::Chinese => "zh".to_string(),
            TargetLanguage::Hebrew => "he".to_string(),
            TargetLanguage::Esperanto => "eo".to_string(),
            TargetLanguage::Persian => "fa".to_string(),
            TargetLanguage::Hawaiian => "haw".to_string(),
        }
    }

    pub fn to_wiktionary_long_name(&self) -> &'static str {
        match self {
            TargetLanguage::Bulgarian => "Bulgarian",
            TargetLanguage::Czech => "Czech",
            TargetLanguage::Danish => "Danish",
            TargetLanguage::German => "German",
            TargetLanguage::Greek => "Greek",
            TargetLanguage::Spanish => "Spanish",
            TargetLanguage::Estonian => "Estonian",
            TargetLanguage::Finnish => "Finnish",
            TargetLanguage::French => "French",
            TargetLanguage::Hungarian => "Hungarian",
            TargetLanguage::Indonesian => "Indonesian",
            TargetLanguage::Italian => "Italian",
            TargetLanguage::Japanese => "Japanese",
            TargetLanguage::Korean => "Korean",
            TargetLanguage::Lithuanian => "Lithuanian",
            TargetLanguage::Latvian => "Latvian",
            TargetLanguage::Norwegian => "Norwegian",
            TargetLanguage::Dutch => "Dutch",
            TargetLanguage::Polish => "Polish",
            TargetLanguage::PortuguesePortugal => "Portuguese (Portugal)",
            TargetLanguage::PortugueseBrazil => "Portuguese (Brazil)",
            TargetLanguage::Romanian => "Romanian",
            TargetLanguage::Russian => "Russian",
            TargetLanguage::Slovakian => "Slovakian",
            TargetLanguage::Slovenian => "Slovenian",
            TargetLanguage::Swedish => "Swedish",
            TargetLanguage::Turkish => "Turkish",
            TargetLanguage::Ukrainian => "Ukrainian",
            TargetLanguage::Chinese => "Chinese",
            TargetLanguage::Hebrew => "Hebrew",
            TargetLanguage::Esperanto => "Esperanto",
            TargetLanguage::Persian => "Persian",
            TargetLanguage::Hawaiian => "Hawaiian",
        }
    }

    pub fn to_nice_format(&self) -> &'static str {
        match &self {
            //TargetLanguage::Arabic => "Arabic",
            TargetLanguage::Bulgarian => "Bulgarian",
            TargetLanguage::Czech => "Czech",
            TargetLanguage::Danish => "Danish",
            TargetLanguage::German => "German",
            TargetLanguage::Greek => "Greek",
            TargetLanguage::Spanish => "Spanish",
            TargetLanguage::Estonian => "Estonian",
            TargetLanguage::Finnish => "Finnish",
            TargetLanguage::French => "French",
            TargetLanguage::Hungarian => "Hungarian",
            TargetLanguage::Indonesian => "Indonesian",
            TargetLanguage::Italian => "Italian",
            TargetLanguage::Japanese => "Japanese",
            TargetLanguage::Korean => "Korean",
            TargetLanguage::Lithuanian => "Lithuanian",
            TargetLanguage::Latvian => "Latvian",
            TargetLanguage::Norwegian => "Norwegian",
            TargetLanguage::Dutch => "Dutch",
            TargetLanguage::Polish => "Polish",
            TargetLanguage::PortuguesePortugal => "PortuguesePortugal",
            TargetLanguage::PortugueseBrazil => "PortugueseBrazil",
            TargetLanguage::Romanian => "Romanian",
            TargetLanguage::Russian => "Russian",
            TargetLanguage::Slovakian => "Slovakian",
            TargetLanguage::Slovenian => "Slovenian",
            TargetLanguage::Swedish => "Swedish",
            TargetLanguage::Turkish => "Turkish",
            TargetLanguage::Ukrainian => "Ukrainian",
            TargetLanguage::Chinese => "Chinese",
            TargetLanguage::Hebrew => "Hebrew",
            TargetLanguage::Esperanto => "Esperanto",
            TargetLanguage::Persian => "Persian",
            TargetLanguage::Hawaiian => "Hawaiian",
        }
    }

    pub fn to_deepl_format(&self) -> &str {
        match self {
            //TargetLanguage::Arabic => "AR",
            TargetLanguage::Bulgarian => "BG",
            TargetLanguage::Czech => "CS",
            TargetLanguage::Danish => "DA",
            TargetLanguage::German => "DE",
            TargetLanguage::Greek => "EL",
            TargetLanguage::Spanish => "ES",
            TargetLanguage::Estonian => "ET",
            TargetLanguage::Finnish => "FI",
            TargetLanguage::French => "FR",
            TargetLanguage::Hungarian => "HU",
            TargetLanguage::Indonesian => "ID",
            TargetLanguage::Italian => "IT",
            TargetLanguage::Japanese => "JA",
            TargetLanguage::Korean => "KO",
            TargetLanguage::Lithuanian => "LT",
            TargetLanguage::Latvian => "LV",
            TargetLanguage::Norwegian => "NB",
            TargetLanguage::Dutch => "NL",
            TargetLanguage::Polish => "PL",
            TargetLanguage::PortuguesePortugal => "PT-PT",
            TargetLanguage::PortugueseBrazil => "PT-BR",
            TargetLanguage::Romanian => "RO",
            TargetLanguage::Russian => "RU",
            TargetLanguage::Slovakian => "SK",
            TargetLanguage::Slovenian => "SL",
            TargetLanguage::Swedish => "SV",
            TargetLanguage::Turkish => "TR",
            TargetLanguage::Ukrainian => "UK",
            TargetLanguage::Chinese => "ZH",
            TargetLanguage::Hebrew => "HE",
            TargetLanguage::Esperanto => "EO",
            TargetLanguage::Persian => "FA",
            TargetLanguage::Hawaiian => "HAW",
        }
    }
}
