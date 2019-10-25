#![allow(dead_code)]
#![allow(nonstandard_style)]
use std::ffi::c_void;

// I pulled these fron std::os::raw
pub type c_char = u8;
pub type c_double = f64;
pub type c_float = f32;
pub type c_int = i32;
pub type c_long = i64;
pub type c_longlong = i64;
pub type c_schar = i8;
pub type c_short = i16;
pub type c_uchar = u8;
pub type c_uint = u32;
pub type c_ulong = u64;
pub type c_ulonglong = u64;
pub type c_ushort = u16;


// I pulled these from stdlib source: libstd::sys::windows::c
// NOTE: the sys module in stdlib is private...
// Normal people should use the winapi crate, but I don't like dependencies
pub type DWORD = c_ulong;
pub type HANDLE = LPVOID;
pub type HINSTANCE = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HRESULT = LONG;
pub type BOOL = c_int;
pub type BYTE = u8;
pub type BOOLEAN = BYTE;
pub type GROUP = c_uint;
pub type LARGE_INTEGER = c_longlong;
pub type LONG = c_long;
pub type UINT = c_uint;
pub type WCHAR = u16;
pub type USHORT = c_ushort;
pub type SIZE_T = usize;
pub type WORD = u16;
pub type CHAR = c_char;
pub type ULONG_PTR = usize;
pub type ULONG = c_ulong;
pub type LPBOOL = *mut BOOL;
pub type LPBYTE = *mut BYTE;
pub type LPCSTR = *const CHAR;
pub type LPCWSTR = *const WCHAR;
pub type LPDWORD = *mut DWORD;
pub type LPHANDLE = *mut HANDLE;
pub type LPVOID = *mut c_void;
pub type LPWCH = *mut WCHAR;
pub type LPSTR = *mut CHAR;
pub type LPWSTR = *mut WCHAR;
pub type PLARGE_INTEGER = *mut c_longlong;