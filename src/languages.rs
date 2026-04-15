use std::collections::HashMap;

/// Represents the configuration for a programming language.
/// Includes file extensions and comment syntax rules.
#[derive(Debug)]
pub struct LanguageConfig {
    pub single_line_comment: Option<&'static str>,
    pub multi_line_comment_start: Option<&'static str>,
    pub multi_line_comment_end: Option<&'static str>,
}

/// A map of supported languages and their configurations.
/// Add new languages here to extend support.
pub static LANGUAGES: phf::Map<&'static str, LanguageConfig> = phf::phf_map! {
    "rs" => LanguageConfig {
        single_line_comment: Some("//"),
        multi_line_comment_start: Some("/*"),
        multi_line_comment_end: Some("*/"),
    },
    "txt" => LanguageConfig {
        single_line_comment: None,
        multi_line_comment_start: None,
        multi_line_comment_end: None,
    },
    "md" => LanguageConfig {
        single_line_comment: None,
        multi_line_comment_start: None,
        multi_line_comment_end: None,
    },
    // Add more languages here
};