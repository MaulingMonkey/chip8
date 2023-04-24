#![cfg(target_arch = "wasm32")]

use crate::*;

use core::cell::*;



thread_local! { static IOBUFFERS : RefCell<[Vec<u8>; 3]> = Default::default(); }

#[export_name = "wasi_snapshot_preview1.fd_write"]
extern "C" fn fd_write(fd: u32, ciovs_ptr: *const Ciovec, ciovs_len: usize, o_size: &mut usize) -> u32 {
    if fd == 0 { return 8 } // BADF

    IOBUFFERS.with(|iobuffers| {
        let mut iobuffers = iobuffers.borrow_mut();

        // accumulate
        let Some(iobuffer) = iobuffers.get_mut(fd as usize) else { return 8 }; // BADF
        let start = iobuffer.len();
        let ciovs = unsafe { core::slice::from_raw_parts(ciovs_ptr, ciovs_len) };
        for ciovec in ciovs.iter().copied() {
            let ciovec = unsafe { core::slice::from_raw_parts(ciovec.buf, ciovec.len) };
            iobuffer.extend_from_slice(ciovec);
        }
        let end = iobuffer.len();
        *o_size = end - start;

        // deaccumulate
        let mut start = 0;
        let mut lines = iobuffer.split_inclusive(|b| *b == b'\n');
        let _incomplete_line = lines.next_back();
        for line in lines {
            let len = line.len();
            start += len;
            match fd {
                1 => console::log(  &line[..len-1]),
                2 => console::error(&line[..len-1]),
                _ => console::error(&line[..len-1]),
            }
        }
        iobuffer.drain(..start);

        return 0; // SUCCESS
    })
}

#[derive(Clone, Copy)] #[repr(C)] pub struct Ciovec {
    pub buf: *const u8,
    pub len: usize,
}
