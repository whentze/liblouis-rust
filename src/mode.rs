use std::os::raw::c_int;
use louis_sys::{translationModes_dotsIO, translationModes_ucBrl, translationModes_noContractions};

pub type TranslationMode = c_int;

pub const DOTS_LOUIS : TranslationMode = translationModes_dotsIO as c_int;
pub const DOTS_UNICODE : TranslationMode = (translationModes_dotsIO | translationModes_ucBrl) as c_int;
pub const NO_CONTRACTIONS : TranslationMode = translationModes_noContractions as c_int;