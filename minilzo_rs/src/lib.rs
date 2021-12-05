// These minilzo bindings are a modified version of [minilzo-rs](https://github.com/badboy/minilzo-rs).

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
extern crate core;

use alloc::vec::Vec;

use core::mem::MaybeUninit;
use core::{fmt, ptr};

use libc::{c_int, c_uchar, c_ulong, c_void};

extern "C" {
    fn lzo1x_1_compress(
        src: *const c_uchar,
        src_len: c_ulong,
        dst: *mut c_uchar,
        dst_len: *mut c_ulong,
        wrkmem: *mut c_void,
    ) -> c_int;
    fn lzo1x_decompress_safe(
        src: *const c_uchar,
        src_len: c_ulong,
        dst: *mut c_uchar,
        dst_len: *mut c_ulong,
        wrkmem: *mut c_void,
    ) -> c_int;
}

const LZO1X_1_MEM_COMPRESS: usize = 16384 * 8;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Error {
    Error = -1,
    OutOfMemory = -2,
    NotCompressible = -3,
    InputOverrun = -4,
    OutputOverrun = -5,
    LookbehindOverrun = -6,
    EOFNotFound = -7,
    InputNotConsumed = -8,
    NotYetImplemented = -9,
    InvalidArgument = -10,
    InvalidAlignment = -11,
    OutputNotConsumed = -12,
    InternalError = -99,
    Other,
}

impl Error {
    pub fn from_code(code: i32) -> Error {
        match code {
            -1 => Error::Error,
            -2 => Error::OutOfMemory,
            -3 => Error::NotCompressible,
            -4 => Error::InputOverrun,
            -5 => Error::OutputOverrun,
            -6 => Error::LookbehindOverrun,
            -7 => Error::EOFNotFound,
            -8 => Error::InputNotConsumed,
            -9 => Error::NotYetImplemented,
            -10 => Error::InvalidArgument,
            -11 => Error::InvalidAlignment,
            -12 => Error::OutputNotConsumed,
            -99 => Error::InternalError,
            _ => Error::Other,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn compress(indata: &[u8]) -> Result<Vec<u8>, Error> {
    let mut wrkmem: MaybeUninit<[u8; LZO1X_1_MEM_COMPRESS]> = MaybeUninit::uninit();

    let inlen = indata.len();
    let outlen = inlen + inlen / 16 + 64 + 3;
    let mut outdata = Vec::with_capacity(outlen);

    unsafe {
        let r = lzo1x_1_compress(
            indata.as_ptr(),
            inlen as c_ulong,
            outdata.as_mut_ptr(),
            &outlen as *const _ as *mut _,
            wrkmem.as_mut_ptr() as *mut _,
        );

        if r == 0 {
            outdata.set_len(outlen);
            return Ok(outdata);
        }

        return Err(Error::from_code(r));
    }
}

pub fn decompress(indata: &[u8], newlen: usize) -> Result<Vec<u8>, Error> {
    let inlen = indata.len();
    let mut outdata = Vec::with_capacity(newlen);

    unsafe {
        let r = lzo1x_decompress_safe(
            indata.as_ptr(),
            inlen as c_ulong,
            outdata.as_mut_ptr(),
            &newlen as *const _ as *mut _,
            ptr::null_mut(),
        );

        if r == 0 {
            outdata.set_len(newlen);
            return Ok(outdata);
        }

        return Err(Error::from_code(r));
    }
}
