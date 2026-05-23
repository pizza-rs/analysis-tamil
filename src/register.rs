//! Registration of Tamil analysis components into the analysis factory.

use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::LowercaseTokenFilter;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use pizza_analysis_core::DecimalDigitTokenFilter;
use pizza_analysis_core::IndicNormalizationTokenFilter;
use pizza_analysis_core::TamilStemTokenFilter;

use crate::normalize::TamilNormalizationFilter;
use crate::stop::TamilStopFilter;

/// Register all Tamil analysis components.
///
/// Registers:
/// - `"tamil"` analyzer (override: indic_norm → tamil_norm → lowercase → decimal_digit → stop → stem)
/// - `"tamil_normalization"` token filter
/// - `"tamil_stop"` token filter
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter(
        "tamil_normalization",
        Box::new(TamilNormalizationFilter::new()),
    );
    factory.register_token_filter("tamil_stop", Box::new(TamilStopFilter::new()));

    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(IndicNormalizationTokenFilter::new()),
        Box::new(TamilNormalizationFilter::new()),
        Box::new(LowercaseTokenFilter::new()),
        Box::new(DecimalDigitTokenFilter::new()),
        Box::new(TamilStopFilter::new()),
        Box::new(TamilStemTokenFilter::new()),
    ];

    factory.register_analyzer(
        "tamil",
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("tamil_normalization").is_some());
        assert!(factory.get_token_filter("tamil_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("tamil").is_some());
    }

    #[test]
    fn test_end_to_end() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("tamil").unwrap();
        let mut text = String::from("மற்றும் தமிழ் மொழி");
        let tokens = analyzer.analyze_and_return_tokens(&mut text);
        // "மற்றும்" is a stop word, should be filtered out
        let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
        assert!(!terms.contains(&"மற்றும்"));
        assert!(!terms.is_empty());
    }

    #[test]
    fn test_tamil_digit_in_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("tamil").unwrap();
        let mut text = String::from("எண் ௧௨௩");
        let tokens = analyzer.analyze_and_return_tokens(&mut text);
        let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
        assert!(terms.contains(&"123"));
    }
}
