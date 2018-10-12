use louis_sys::{
    translationModes_dotsIO, translationModes_noContractions, translationModes_partialTrans,
    translationModes_ucBrl,
};

pub type TranslationModes = std::os::raw::c_int;

/// Output Braille dots using Unicode
pub const DOTS_UNICODE: TranslationModes =
    (translationModes_dotsIO | translationModes_ucBrl) as TranslationModes;

/// Output Braille dots using liblouis' own encoding
pub const DOTS_LOUIS: TranslationModes = translationModes_dotsIO as TranslationModes;

/// Do not perform any contractions
pub const NO_CONTRACTIONS: TranslationModes = translationModes_noContractions as TranslationModes;

pub const PARTIAL_TRANS: TranslationModes = translationModes_partialTrans as TranslationModes;
