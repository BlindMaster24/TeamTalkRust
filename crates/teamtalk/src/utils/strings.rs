//! Helpers for TeamTalk string conversions.
use std::borrow::Cow;
use teamtalk_sys as ffi;

#[cfg(not(windows))]
fn ttchar_bytes(slice: &[ffi::TTCHAR]) -> &[u8] {
    // Safety: on non-Windows TTCHAR is `char` (1 byte). We only reinterpret the same length.
    unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) }
}

/// Creates a zeroed TeamTalk string buffer.
pub fn tt_buf<const N: usize>() -> [ffi::TTCHAR; N] {
    [0 as ffi::TTCHAR; N]
}

#[allow(clippy::large_enum_variant)]
/// A stack-allocated or heap-allocated TeamTalk string.
///
/// TeamTalk typically uses fixed buffers of 512 bytes. This structure
/// avoids heap allocations for strings within that limit.
pub enum TTString {
    Stack([ffi::TTCHAR; 512], usize),
    Heap(Vec<ffi::TTCHAR>),
}

impl TTString {
    pub fn as_ptr(&self) -> *const ffi::TTCHAR {
        match self {
            Self::Stack(arr, _) => arr.as_ptr(),
            Self::Heap(v) => v.as_ptr(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Stack(_, len) => *len,
            Self::Heap(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[ffi::TTCHAR] {
        match self {
            Self::Stack(arr, len) => &arr[..*len],
            Self::Heap(v) => v.as_slice(),
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [ffi::TTCHAR] {
        match self {
            Self::Stack(arr, len) => &mut arr[..*len],
            Self::Heap(v) => v.as_mut_slice(),
        }
    }
}

impl std::ops::Deref for TTString {
    type Target = [ffi::TTCHAR];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl std::ops::DerefMut for TTString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl AsRef<[ffi::TTCHAR]> for TTString {
    fn as_ref(&self) -> &[ffi::TTCHAR] {
        self.as_slice()
    }
}

/// Converts Rust strings into TeamTalk UTF-16 or UTF-8 buffers.
pub trait ToTT {
    fn tt(&self) -> TTString;
}

impl ToTT for str {
    fn tt(&self) -> TTString {
        #[cfg(windows)]
        {
            let mut len = 0;
            let mut buf = [0u16; 512];
            for (i, c) in self.encode_utf16().enumerate() {
                if i >= 511 {
                    return TTString::Heap(self.encode_utf16().chain(std::iter::once(0)).collect());
                }
                buf[i] = c;
                len = i + 1;
            }
            buf[len] = 0;
            TTString::Stack(buf, len + 1)
        }
        #[cfg(not(windows))]
        {
            let bytes = self.as_bytes();
            if bytes.len() >= 511 {
                let mut v: Vec<i8> = bytes.iter().map(|&b| b as i8).collect();
                v.push(0);
                return TTString::Heap(v);
            }
            let mut buf = [0i8; 512];
            for (i, &b) in bytes.iter().enumerate() {
                buf[i] = b as i8;
            }
            buf[bytes.len()] = 0;
            TTString::Stack(buf, bytes.len() + 1)
        }
    }
}

impl ToTT for String {
    fn tt(&self) -> TTString {
        self.as_str().tt()
    }
}

/// Converts a raw TeamTalk string pointer into `String`.
///
/// # Safety
/// `ptr` must be a valid null-terminated TeamTalk string.
pub unsafe fn from_tt(ptr: *const ffi::TTCHAR) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let mut len = 0;
    unsafe {
        while *ptr.add(len) != 0 {
            len += 1;
        }
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    #[cfg(windows)]
    {
        String::from_utf16_lossy(slice)
    }
    #[cfg(not(windows))]
    {
        String::from_utf8_lossy(ttchar_bytes(slice)).into_owned()
    }
}

/// Converts a TeamTalk string buffer into `String`.
pub fn to_string(arr: &[ffi::TTCHAR]) -> String {
    let len = arr.iter().position(|&c| c == 0).unwrap_or(arr.len());
    #[cfg(windows)]
    {
        String::from_utf16_lossy(&arr[..len])
    }
    #[cfg(not(windows))]
    {
        String::from_utf8_lossy(ttchar_bytes(&arr[..len])).into_owned()
    }
}

/// Converts a TeamTalk string buffer into a `Cow<str>`.
pub fn to_cow(arr: &[ffi::TTCHAR]) -> Cow<'_, str> {
    let len = arr.iter().position(|&c| c == 0).unwrap_or(arr.len());
    #[cfg(windows)]
    {
        Cow::Owned(String::from_utf16_lossy(&arr[..len]))
    }
    #[cfg(not(windows))]
    {
        String::from_utf8_lossy(ttchar_bytes(&arr[..len]))
    }
}

/// Copies a TeamTalk string buffer into a reusable `String`.
pub fn copy_to_string(arr: &[ffi::TTCHAR], out: &mut String) {
    out.clear();
    let len = arr.iter().position(|&c| c == 0).unwrap_or(arr.len());
    #[cfg(windows)]
    {
        *out = String::from_utf16_lossy(&arr[..len]);
    }
    #[cfg(not(windows))]
    {
        out.push_str(&String::from_utf8_lossy(ttchar_bytes(&arr[..len])));
    }
}
