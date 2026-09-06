//! Tamil-specific text normalization.
//!
//! Normalizes Tamil text beyond what `IndicNormalizationFilter` handles:
//! - Zero-width character removal (ZWJ, ZWNJ, ZW-Space)
//! - Tamil digit normalization (௦-௯ → 0-9)
//! - Old-style Tamil numeral signs removal (௰, ௱, ௲)

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Tamil-specific normalization filter.
///
/// Complements `IndicNormalizationFilter` with Tamil-specific rules:
/// - Removes zero-width characters (U+200B, U+200C, U+200D)
/// - Normalizes Tamil digits ௦-௯ (U+0BE6-U+0BEF) to ASCII 0-9
/// - Removes old Tamil numeral signs ௰ (ten), ௱ (hundred), ௲ (thousand)
#[derive(Clone, Debug, Default)]
pub struct TamilNormalizationFilter;

impl TamilNormalizationFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for TamilNormalizationFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.is_empty() {
            return (false, None);
        }

        // Fast path: check if any Tamil-range or zero-width chars present
        let needs_work = text.chars().any(|c| {
            let cp = c as u32;
            (0x0BE6..=0x0BF2).contains(&cp) // Tamil digits + numeral signs
                || (0x200B..=0x200D).contains(&cp) // Zero-width chars
        });

        if !needs_work {
            return (false, None);
        }

        let mut result = String::with_capacity(text.len());
        let mut changed = false;

        for c in text.chars() {
            let cp = c as u32;
            match cp {
                // Zero-width space, ZWNJ, ZWJ — remove
                0x200B..=0x200D => {
                    changed = true;
                }
                // Tamil digits ௦-௯ → ASCII 0-9
                0x0BE6..=0x0BEF => {
                    result.push(char::from(b'0' + (cp - 0x0BE6) as u8));
                    changed = true;
                }
                // Old Tamil numeral signs ௰ (ten), ௱ (hundred), ௲ (thousand) — remove
                0x0BF0..=0x0BF2 => {
                    changed = true;
                }
                _ => {
                    result.push(c);
                }
            }
        }

        if changed {
            token.term = Cow::Owned(result);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pizza_engine::analysis::Token;

    fn make_token(term: &str) -> Token {
        Token {
            term: Cow::Owned(term.to_string()),
            start_offset: 0,
            end_offset: term.len() as u32,
            position: 0,
        }
    }

    #[test]
    fn test_tamil_digit_normalization() {
        let filter = TamilNormalizationFilter::new();
        let mut token = make_token("௧௨௩");
        filter.filter(&mut token);
        assert_eq!(token.term.as_ref(), "123");
    }

    #[test]
    fn test_zero_width_removal() {
        let filter = TamilNormalizationFilter::new();
        let mut token = make_token("தமி\u{200D}ழ்");
        filter.filter(&mut token);
        assert_eq!(token.term.as_ref(), "தமிழ்");
    }

    #[test]
    fn test_numeral_sign_removal() {
        let filter = TamilNormalizationFilter::new();
        let mut token = make_token("௲");
        filter.filter(&mut token);
        assert_eq!(token.term.as_ref(), "");
    }

    #[test]
    fn test_no_change_plain_tamil() {
        let filter = TamilNormalizationFilter::new();
        let mut token = make_token("தமிழ்");
        filter.filter(&mut token);
        assert_eq!(token.term.as_ref(), "தமிழ்");
    }

    #[test]
    fn test_mixed_content() {
        let filter = TamilNormalizationFilter::new();
        let mut token = make_token("விலை௧௦௰");
        filter.filter(&mut token);
        assert_eq!(token.term.as_ref(), "விலை10");
    }
}
