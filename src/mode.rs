use louis_sys::{translationModes_dotsIO, translationModes_noContractions, translationModes_ucBrl};
use std::os::raw::c_int;

pub type TranslationMode = c_int;

/// Output Braille dots using Unicode
pub const DOTS_UNICODE: TranslationMode =
    (translationModes_dotsIO | translationModes_ucBrl) as c_int;

/// Output Braille dots using liblouis' own encoding
pub const DOTS_LOUIS: TranslationMode = translationModes_dotsIO as c_int;

/// Do not perform any contractions
pub const NO_CONTRACTIONS: TranslationMode = translationModes_noContractions as c_int;
