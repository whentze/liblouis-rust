#![feature(dbg_macro)]

use lazy_static::lazy_static;
use louis_sys::ThreadUnsafetyToken;
use std::ffi::{CStr, CString};
use std::mem::drop;
use std::os::raw::{c_int, c_uint};
use std::path::Path;
use std::sync::Mutex;

pub mod mode;

lazy_static! {
    pub static ref TOKEN: Mutex<ThreadUnsafetyToken> =
        Mutex::new(ThreadUnsafetyToken::take().expect("Could not get ThreadUnsafetyToken."));
}

type LouisString = widestring::UCString<louis_sys::widechar>;

pub fn liblouis_version() -> Result<semver::Version, semver::SemVerError> {
    let guard = TOKEN.lock();
    let version_str = unsafe { CStr::from_ptr(louis_sys::lou_version()) }
        .to_str()
        .unwrap();
    drop(guard);
    semver::Version::parse(version_str)
}

pub fn list_tables() -> Vec<String> {
    let guard = TOKEN.lock();
    let list_begin = unsafe { louis_sys::lou_listTables() };
    drop(guard);
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

const OUTLEN_MULTIPLIER : c_int = 4 + 2*std::mem::size_of::<louis_sys::widechar>() as c_int;

pub fn translate_simple(table_name: &str, input: &str, mode: mode::TranslationMode) -> String {
    let inbuf = LouisString::from_str(input).unwrap();
    let mut inlen = inbuf.len() as c_int;

    let mut outlen = inlen * OUTLEN_MULTIPLIER;
    let mut outvec = Vec::with_capacity(outlen as usize);
    let outptr = outvec.as_mut_ptr();

    let guard = TOKEN.lock();
    unsafe {
        louis_sys::lou_translateString(
            CString::new(table_name).unwrap().as_ptr(),
            inbuf.as_ptr(),
            &mut inlen as *mut _,
            outptr,
            &mut outlen as *mut _,
            std::ptr::null::<louis_sys::formtype>() as *mut _,
            std::ptr::null::<std::os::raw::c_char>() as *mut _,
            mode,
        )
    };
    drop(guard);

    std::mem::forget(outvec);
    let outbuf = unsafe { LouisString::from_ptr(outptr, outlen as usize) }.unwrap();
    outbuf.to_string().unwrap()
}

fn lou_loglevel_to_level(level: c_uint) -> log::Level{
    match level {
        0...louis_sys::logLevels_LOG_ALL   => log::Level::Trace,
        0...louis_sys::logLevels_LOG_DEBUG => log::Level::Debug,
        0...louis_sys::logLevels_LOG_INFO  => log::Level::Info,
        0...louis_sys::logLevels_LOG_WARN  => log::Level::Warn,
        _                                  => log::Level::Error,
    }
}

fn filter_to_lou_loglevel(filter: log::LevelFilter) -> c_uint {
    match filter {
        log::LevelFilter::Trace => louis_sys::logLevels_LOG_ALL,
        log::LevelFilter::Debug => louis_sys::logLevels_LOG_DEBUG,
        log::LevelFilter::Info  => louis_sys::logLevels_LOG_INFO,
        log::LevelFilter::Warn  => louis_sys::logLevels_LOG_WARN,
        log::LevelFilter::Error => louis_sys::logLevels_LOG_ERROR,
        log::LevelFilter::Off   => louis_sys::logLevels_LOG_OFF,
    }
}

pub fn enable_logging() {
    let guard = TOKEN.lock();
    unsafe {
        louis_sys::lou_setLogLevel(filter_to_lou_loglevel(log::STATIC_MAX_LEVEL));
        louis_sys::lou_registerLogCallback(Some(log_callback));
    };
    drop(guard);
}

unsafe extern "C" fn log_callback(level: louis_sys::logLevels, message: *const ::std::os::raw::c_char) {
    let message_str = CStr::from_ptr(message).to_string_lossy();
    log::log!(target: "liblouis", lou_loglevel_to_level(level), "{}", message_str);
}

#[cfg(test)]
mod tests;
