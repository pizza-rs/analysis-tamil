//! Tamil text analysis plugin for INFINI Pizza.
//!
//! Provides a dedicated `"tamil"` analyzer that overrides the basic analyzer
//! from `analysis-core` with a richer pipeline:
//!
//! ```text
//! StandardTokenizer → IndicNormalization → TamilNormalization → Lowercase
//!     → DecimalDigit → Stop → TamilStem
//! ```
//!
//! ## Components
//!
//! - **TamilNormalizationFilter** — Tamil-specific normalizations:
//!   - Aaytham (ஃ) removal in certain contexts
//!   - Old-style ligature normalization
//!   - Zero-width character removal
//! - **TamilStopFilter** — 100+ Tamil stop words with O(1) lookup
//! - Reuses `IndicNormalizationFilter`, `TamilStemTokenFilter` from `analysis-core`

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod normalize;
mod register;
mod stop;

pub use normalize::TamilNormalizationFilter;
pub use register::register_all;
pub use stop::TamilStopFilter;
