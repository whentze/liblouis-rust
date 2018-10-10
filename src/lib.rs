#![feature(dbg_macro)]

use lazy_static::lazy_static;
use louis_sys::ThreadUnsafetyToken;
use std::ffi::{CStr, CString};
use std::mem::drop;
use std::path::Path;
use std::sync::Mutex;

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

pub fn translate_simple(table_name: &str, input: &str) -> String {
    let inbuf = LouisString::from_str(input).unwrap();
    let mut inlen = inbuf.len() as std::os::raw::c_int;

    let mut outvec = Vec::with_capacity(inbuf.len());
    let mut outlen = inlen;
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
            0,
        )
    };
    drop(guard);

    std::mem::forget(outvec);
    let outbuf = unsafe { LouisString::from_ptr(outptr, outlen as usize) }.unwrap();
    outbuf.to_string().unwrap()
}

#[cfg(test)]
mod tests;
