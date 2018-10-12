#![feature(dbg_macro)]

use louis_sys::ThreadUnsafetyToken;
use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int, c_uint};
use std::path::Path;

pub mod modes;

type LouisString = widestring::UCString<louis_sys::widechar>;
const OUTLEN_MULTIPLIER: c_int = 4 + 2 * std::mem::size_of::<louis_sys::widechar>() as c_int;

/// A singleton that handles all access to liblouis.
///
/// This struct is needed since liblouis is thread-unsafe and can only be called from one thread at a time.
/// It is `Send`, but `!Sync`, so that at any given moment, all `&`s to it live on the same thread,
/// but if you own it you can move it across threads. All liblouis calls therefore need a `&` to this.
/// It also provides some convenient setup/teardown logic:
/// - When created, it registers a logging callback with liblouis that
///   pipes all messages into the log.rs facade with the appropriate log levels set
/// - When dropped, it resets liblouis' logging behaviour to the default and calls `lou_free()`
///   to make sure no memory is leaked.
pub struct Louis {
    _token: ThreadUnsafetyToken,
    nosync: PhantomData<Cell<u8>>,
}

impl Louis {
    /// Tries to initialize liblouis, returning `Some(Louis)` on success.
    /// On failure, it returns `None`, indicating that the ThreadUnsafetyToken has already been taken.
    pub fn new() -> Option<Self> {
        ThreadUnsafetyToken::take().map(|_token| {
            let louis = Louis {
                _token,
                nosync: PhantomData,
            };
            louis.configure_logging();
            louis
        })
    }

    /// Returns the version of liblouis that this crate is linked against
    pub fn version(&self) -> Result<semver::Version, semver::SemVerError> {
        let version_str = unsafe { CStr::from_ptr(louis_sys::lou_version()) }
            .to_str()
            .unwrap();
        semver::Version::parse(version_str)
    }

    /// Lists the filenames of all the tables that are available
    pub fn list_tables(&self) -> Vec<String> {
        let list_begin = unsafe { louis_sys::lou_listTables() };
        let mut res = Vec::new();
        for offset in 0.. {
            let ptr = unsafe { *(list_begin.offset(offset)) };
            if ptr == std::ptr::null() {
                break;
            }
            let table_name = Path::new(unsafe { CStr::from_ptr(ptr) }.to_str().unwrap())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            res.push(table_name);
        }
        res
    }

    /// Translates the text in `input` according to the table given by `table_name`.
    ///
    /// # Examples
    ///
    /// Pass `mode=0` for regular translation:
    ///
    /// ```
    /// # use louis::Louis;
    /// let louis = Louis::new().unwrap();
    /// let brl = louis.translate_simple("ru.tbl", "Я понимаю", false, 0);
    /// assert_eq!(brl, "$ PONIMA|");
    /// ```
    ///
    /// You can also translate directly to Unicode Braille dots:
    ///
    /// ```
    /// # use louis::{Louis, modes::DOTS_UNICODE};
    /// # let louis = Louis::new().unwrap();
    /// let dots = louis.translate_simple("sr.tbl", "Добродошли", false, DOTS_UNICODE);
    /// assert_eq!(dots, "⠨⠙⠕⠃⠗⠕⠙⠕⠱⠇⠊");
    /// ```
    pub fn translate_simple(
        &self,
        table_name: &str,
        input: &str,
        backwards: bool,
        mode: modes::TranslationModes,
    ) -> String {
        let inbuf = LouisString::from_str(input).unwrap();
        let mut inlen = inbuf.len() as c_int;

        let mut outlen = inlen * OUTLEN_MULTIPLIER;
        let mut outvec = Vec::with_capacity(outlen as usize);
        let outptr = outvec.as_mut_ptr();

        unsafe {
            if backwards {
                louis_sys::lou_backTranslateString(
                    CString::new(table_name).unwrap().as_ptr(),
                    inbuf.as_ptr(),
                    &mut inlen as *mut _,
                    outptr,
                    &mut outlen as *mut _,
                    std::ptr::null::<louis_sys::formtype>() as *mut _,
                    std::ptr::null::<std::os::raw::c_char>() as *mut _,
                    mode,
                );
            } else {
                louis_sys::lou_translateString(
                    CString::new(table_name).unwrap().as_ptr(),
                    inbuf.as_ptr(),
                    &mut inlen as *mut _,
                    outptr,
                    &mut outlen as *mut _,
                    std::ptr::null::<louis_sys::formtype>() as *mut _,
                    std::ptr::null::<std::os::raw::c_char>() as *mut _,
                    mode,
                );
            }
        };

        std::mem::forget(outvec);
        let outbuf = unsafe { LouisString::from_ptr(outptr, outlen as usize) }.unwrap();
        outbuf.to_string().unwrap()
    }

    fn configure_logging(&self) {
        unsafe {
            louis_sys::lou_setLogLevel(filter_to_lou_loglevel(log::STATIC_MAX_LEVEL));
            louis_sys::lou_registerLogCallback(Some(log_callback));
        };
    }

    fn reset_logging(&self) {
        unsafe {
            louis_sys::lou_setLogLevel(louis_sys::logLevels_LOG_INFO);
            louis_sys::lou_registerLogCallback(None);
        };
    }
}

impl Drop for Louis {
    fn drop(&mut self) {
        self.reset_logging();
        unsafe { louis_sys::lou_free() };
    }
}

fn lou_loglevel_to_level(level: c_uint) -> log::Level {
    match level {
        0...louis_sys::logLevels_LOG_ALL => log::Level::Trace,
        0...louis_sys::logLevels_LOG_DEBUG => log::Level::Debug,
        0...louis_sys::logLevels_LOG_INFO => log::Level::Info,
        0...louis_sys::logLevels_LOG_WARN => log::Level::Warn,
        _ => log::Level::Error,
    }
}

fn filter_to_lou_loglevel(filter: log::LevelFilter) -> c_uint {
    match filter {
        log::LevelFilter::Trace => louis_sys::logLevels_LOG_ALL,
        log::LevelFilter::Debug => louis_sys::logLevels_LOG_DEBUG,
        log::LevelFilter::Info => louis_sys::logLevels_LOG_INFO,
        log::LevelFilter::Warn => louis_sys::logLevels_LOG_WARN,
        log::LevelFilter::Error => louis_sys::logLevels_LOG_ERROR,
        log::LevelFilter::Off => louis_sys::logLevels_LOG_OFF,
    }
}

unsafe extern "C" fn log_callback(level: louis_sys::logLevels, message: *const c_char) {
    let message_str = CStr::from_ptr(message).to_string_lossy();
    log::log!(target: "liblouis", lou_loglevel_to_level(level), "{}", message_str);
}

#[cfg(test)]
mod tests;
