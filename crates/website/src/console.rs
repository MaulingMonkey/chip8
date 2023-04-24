#![cfg(target_arch = "wasm32")]
#![allow(dead_code)]

#[link(wasm_import_module = "console")] extern "C" {
    #[link_name = "error"   ] fn ffi_error( msg: *const u8, len: usize);
    #[link_name = "log"     ] fn ffi_log(   msg: *const u8, len: usize);
    #[link_name = "panic"   ] fn ffi_panic( msg: *const u8, len: usize);
}

pub fn error(msg: impl AsRef<[u8]>) { let msg = msg.as_ref(); unsafe { ffi_error(msg.as_ptr(), msg.len()) } }
pub fn log(  msg: impl AsRef<[u8]>) { let msg = msg.as_ref(); unsafe { ffi_log(  msg.as_ptr(), msg.len()) } }
pub fn panic(msg: impl AsRef<[u8]>) { let msg = msg.as_ref(); unsafe { ffi_panic(msg.as_ptr(), msg.len()) } }
