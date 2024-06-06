use std::ffi::CStr;
use std::os::raw::c_char;
use std::str;

pub fn format_size(format: u16) -> u32 {
    match format {
        1 | 2  | 6 | 7 => 1,
        3 | 8          => 2,
        4 | 9  | 11    => 4,
        5 | 10 | 12    => 8,
        _ => 0
    }
}

pub fn fetch_u16(data: &[u8], offset: usize, is_le: bool) -> u16 {
    let info = data[offset..(offset + 2)].try_into().unwrap();
    if is_le {
        u16::from_le_bytes(info)
    } else {
        u16::from_be_bytes(info)
    }
}

pub fn fetch_u32(data: &[u8], offset: usize, is_le: bool) -> u32 {
    let info = data[offset..(offset + 4)].try_into().unwrap();
    if is_le {
        u32::from_le_bytes(info)
    } else {
        u32::from_be_bytes(info)
    }
}

pub fn fetch_rational_str(data: &[u8], offset: usize, is_le : bool) -> String {
    let num = fetch_u32(data, offset, is_le);
    let den = fetch_u32(data, offset + 4, is_le);
    format!("{}/{}", num, den)
}

pub fn fetch_null_terminated_str(data: &[u8], offset: usize) -> &str {
    let offset_ptr = unsafe { data.as_ptr().add(offset) };
    let c_str: &CStr = unsafe { CStr::from_ptr(offset_ptr as *const c_char) };
    c_str.to_str().unwrap()
}