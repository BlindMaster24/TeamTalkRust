use std::borrow::Cow;
use teamtalk_sys as ffi;

pub fn tt_buf<const N: usize>() -> [ffi::TTCHAR; N] {
    [0 as ffi::TTCHAR; N]
}

pub trait ToTT {
    fn tt(&self) -> Vec<ffi::TTCHAR>;
}

impl ToTT for str {
    fn tt(&self) -> Vec<ffi::TTCHAR> {
        #[cfg(windows)]
        {
            let mut v: Vec<u16> = self.encode_utf16().collect();
            v.push(0);
            v
        }
        #[cfg(not(windows))]
        {
            let mut v: Vec<i8> = self.as_bytes().iter().map(|&b| b as i8).collect();
            v.push(0);
            v
        }
    }
}

impl ToTT for String {
    fn tt(&self) -> Vec<ffi::TTCHAR> {
        self.as_str().tt()
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn from_tt(ptr: *const ffi::TTCHAR) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let mut len = 0;
    unsafe {
        while *ptr.add(len) != 0 {
            len += 1;
        }
        let slice = std::slice::from_raw_parts(ptr, len);
        #[cfg(windows)]
        {
            String::from_utf16_lossy(slice)
        }
        #[cfg(not(windows))]
        {
            let u8_slice: &[u8] = std::mem::transmute(slice);
            String::from_utf8_lossy(u8_slice).into_owned()
        }
    }
}

pub fn to_string(arr: &[ffi::TTCHAR]) -> String {
    let len = arr.iter().position(|&c| c == 0).unwrap_or(arr.len());
    #[cfg(windows)]
    {
        String::from_utf16_lossy(&arr[..len])
    }
    #[cfg(not(windows))]
    {
        let u8_slice: &[u8] = unsafe { std::mem::transmute(&arr[..len]) };
        String::from_utf8_lossy(u8_slice).into_owned()
    }
}

pub fn to_cow(arr: &[ffi::TTCHAR]) -> Cow<'_, str> {
    let len = arr.iter().position(|&c| c == 0).unwrap_or(arr.len());
    #[cfg(windows)]
    {
        Cow::Owned(String::from_utf16_lossy(&arr[..len]))
    }
    #[cfg(not(windows))]
    {
        let u8_slice: &[u8] = unsafe { std::mem::transmute(&arr[..len]) };
        String::from_utf8_lossy(u8_slice)
    }
}

pub fn copy_to_string(arr: &[ffi::TTCHAR], out: &mut String) {
    out.clear();
    let len = arr.iter().position(|&c| c == 0).unwrap_or(arr.len());
    #[cfg(windows)]
    {
        *out = String::from_utf16_lossy(&arr[..len]);
    }
    #[cfg(not(windows))]
    {
        let u8_slice: &[u8] = unsafe { std::mem::transmute(&arr[..len]) };
        out.push_str(&String::from_utf8_lossy(u8_slice));
    }
}
