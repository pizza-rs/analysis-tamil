//! Tamil stop word filter.
//!
//! Contains 100+ common Tamil stop words (function words, postpositions,
//! pronouns, conjunctions) for filtering during analysis.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Standard Tamil stop words — particles, postpositions, pronouns, auxiliaries.
static DEFAULT_STOP_WORDS: &[&str] = &[
    // Pronouns
    "நான்", "நீ", "அவன்", "அவள்", "அவர்", "அது", "நாம்", "நாங்கள்",
    "நீங்கள்", "அவர்கள்", "அவை", "இது", "இவன்", "இவள்", "இவர்",
    // Demonstratives & question words
    "எது", "யார்", "எங்கே", "எப்போது", "எப்படி", "ஏன்", "என்ன",
    // Postpositions / case markers
    "இல்", "இன்", "ஐ", "ஆல்", "உடன்", "க்கு", "இடம்", "மேல்",
    "கீழ்", "பின்", "முன்", "வரை", "பற்றி", "மூலம்",
    // Conjunctions & connectors
    "மற்றும்", "அல்லது", "ஆனால்", "ஆகவே", "எனவே", "ஏனென்றால்",
    "அதனால்", "ஆகையால்", "இருந்தாலும்", "என்றாலும்",
    // Auxiliaries & copulas
    "இருக்கிறது", "இருந்தது", "இருக்கும்", "ஆகும்", "ஆகிறது",
    "உள்ளது", "உள்ளன", "இல்லை", "வேண்டும்", "முடியும்",
    "போது", "பின்னர்",
    // Common verb forms
    "செய்", "செய்து", "செய்யும்", "வந்து", "வரும்", "போய்",
    "கொண்டு", "கொண்ட", "என்று", "என்ற", "என்பது",
    // Particles & emphatics
    "ஒரு", "ஒரே", "அந்த", "இந்த", "அதன்", "இதன்",
    "அங்கு", "இங்கு", "அப்போது", "இப்போது",
    "மிகவும்", "மிக", "மட்டும்", "தான்", "கூட", "போல்",
    // Numbers & quantifiers
    "எல்லா", "சில", "பல", "ஒவ்வொரு", "எந்த", "அனைத்து",
    // Common adverbs
    "இங்கே", "அங்கே", "இப்படி", "அப்படி", "மீண்டும்",
    "உடனடியாக", "திடீரென்று",
    // Relative particles
    "ஆகிய", "ஆன", "போன்ற", "பொருட்டு",
    // Other function words
    "ஆக", "ஆகி", "அவ்வாறு", "இவ்வாறு",
    "இதில்", "அதில்", "அதை", "இதை",
];

static STOP_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    DEFAULT_STOP_WORDS.iter().copied().collect()
});

/// Filters out common Tamil stop words.
#[derive(Clone, Debug)]
pub struct TamilStopFilter {
    custom: Option<HashSet<String>>,
}

impl Default for TamilStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TamilStopFilter {
    pub fn new() -> Self {
        Self { custom: None }
    }

    /// Create with a custom stop word list instead of the defaults.
    pub fn with_words(words: &[&str]) -> Self {
        let set: HashSet<String> = words.iter().map(|w| w.to_string()).collect();
        Self {
            custom: Some(set),
        }
    }

    fn is_stop(&self, word: &str) -> bool {
        if let Some(ref custom) = self.custom {
            custom.contains(word)
        } else {
            STOP_SET.contains(word)
        }
    }
}

impl TokenFilter for TamilStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        (self.is_stop(token.term.as_ref()), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_detected() {
        let filter = TamilStopFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("மற்றும்"),
            start_offset: 0,
            end_offset: 0,
            position: 0,
        };
        let (removed, _) = filter.filter(&mut token);
        assert!(removed);
    }

    #[test]
    fn test_non_stop_word() {
        let filter = TamilStopFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("பல்கலைக்கழகம்"),
            start_offset: 0,
            end_offset: 0,
            position: 0,
        };
        let (removed, _) = filter.filter(&mut token);
        assert!(!removed);
    }

    #[test]
    fn test_custom_words() {
        let filter = TamilStopFilter::with_words(&["custom", "words"]);
        let mut token = Token {
            term: Cow::Borrowed("custom"),
            start_offset: 0,
            end_offset: 0,
            position: 0,
        };
        let (removed, _) = filter.filter(&mut token);
        assert!(removed);
    }
}
