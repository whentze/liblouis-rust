use louis_sys::{
    translationModes_dotsIO, translationModes_noContractions, translationModes_partialTrans,
    translationModes_ucBrl,
};

use std::os::raw::c_int;

pub type TranslationModes = c_int;

/// Output Braille dots using Unicode
pub const DOTS_UNICODE: TranslationModes =
    (translationModes_dotsIO | translationModes_ucBrl) as TranslationModes;

/// Output Braille dots using liblouis' own encoding
pub const DOTS_LOUIS: TranslationModes = translationModes_dotsIO as TranslationModes;

/// Do not perform any contractions
pub const NO_CONTRACTIONS: TranslationModes = translationModes_noContractions as TranslationModes;

/// This flag specifies that back-translation input should be treated as an incomplete word.
/// Rules that apply only for complete words or at the end of a word will not take effect.
/// This is intended to be used when translating input typed on a braille keyboard 
/// to provide a rough ideato the user of the characters they are typing before the word is complete. 
pub const PARTIAL_TRANS: TranslationModes = translationModes_partialTrans as TranslationModes;
