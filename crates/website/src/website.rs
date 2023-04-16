use maulingmonkey_chip8_interpreter::*;

use core::cell::RefCell;
use std::cell::RefMut;



thread_local! {
    static CONTEXT : &'static RefCell<Context> = Box::leak(Box::new(Default::default()));
    static CONTEXT_LOCK : RefCell<Option<RefMut<'static, Context>>> = Default::default();
}

#[cfg(target_arch = "wasm32")] mod console {
    #[link(wasm_import_module = "console")] extern "C" {
        #[link_name = "log"     ] fn ffi_log(msg: usize, len: usize);
        #[link_name = "panic"   ] fn ffi_panic(msg: usize, len: usize);
    }

    pub fn log(  msg: impl AsRef<str>) { let msg = msg.as_ref(); unsafe { ffi_log(  msg.as_ptr() as _, msg.len()) } }
    pub fn panic(msg: impl AsRef<str>) { let msg = msg.as_ref(); unsafe { ffi_panic(msg.as_ptr() as _, msg.len()) } }
}

#[no_mangle] pub extern "C" fn setup() {
    #[cfg(target_arch = "wasm32")] std::panic::set_hook(Box::new(|pi|{
        let payload = pi.payload();
        if let Some(s) = payload.downcast_ref::<&str>() {
            console::panic(s);
        } else if let Some(s) = payload.downcast_ref::<String>() {
            console::panic(s);
        } else {
            console::panic(format!("panic with unknown payload: {pi:?}"));
        }
    }));

    context_reset();
}

#[no_mangle] pub extern "C" fn lock_memory_range(start: usize, end: usize) -> usize {
    CONTEXT_LOCK.with(|lock| CONTEXT.with(|context| {
        #[cfg(debug_assertions)] #[cfg(target_arch = "wasm32")] console::log(format!("lock_memory_range({start}, {end})"));
        assert!(lock.borrow().is_none(), "lock_memory_range({start} .. {end}) called, but memory was already locked");
        let mut ref_mut = context.borrow_mut();
        let ptr = ref_mut.memory.as_bytes_mut()[start..end].as_mut_ptr() as usize;
        *lock.borrow_mut() = Some(ref_mut);
        ptr
    }))
}

#[no_mangle] pub extern "C" fn unlock_memory_range() {
    #[cfg(debug_assertions)] #[cfg(target_arch = "wasm32")] console::log(format!("unlock_memory_range()"));
    CONTEXT_LOCK.with(|lock| {
        assert!(lock.borrow().is_some(), "unlock_memory_range() called, but no memory range was locked");
        *lock.borrow_mut() = None;
    })
}

#[no_mangle] pub extern "C" fn context_register_i()         -> Addr { CONTEXT.with(|tls| tls.borrow().registers.i)  }
#[no_mangle] pub extern "C" fn context_register_pc()        -> Addr { CONTEXT.with(|tls| tls.borrow().registers.pc) }
#[no_mangle] pub extern "C" fn context_register_v(v: u32)   -> u8 {
    let v = V(Nibble::try_from(v).unwrap_or_else(|_| panic!("context_register_v({v}) called, out of bounds (max expected was 0xF)")));
    CONTEXT.with(|tls| tls.borrow().registers[v])
}



#[no_mangle] pub extern "C" fn context_reset() {
    let mut ctx = Context::new();
    ctx.registers.pc = Addr::PROGRAM_START_TYPICAL;
    ctx.memory.copy_from_slice(Addr::SYSTEM_INTERPRETER_FONTS_START, bytemuck::cast_slice(font::DEFAULT)).expect("failed to copy font into memory"); // ≈ pointless?
    ctx.memory.copy_from_slice(ctx.registers.pc, include_bytes!("../../../examples/sierpinski.ch8")).expect("failed to copy sierpinski.ch8 ROM into memory");

    CONTEXT.with(|tls| *tls.borrow_mut() = ctx);
}

#[no_mangle] pub extern "C" fn context_try_step_single() -> bool { CONTEXT.with(|tls| tls.borrow_mut().try_step_single()) }
#[no_mangle] pub extern "C" fn context_try_step_many(budget: usize) -> usize { CONTEXT.with(|tls| tls.borrow_mut().try_step_many(budget)) }
#[no_mangle] pub extern "C" fn context_step_clocks(clocks: usize) { CONTEXT.with(|tls| { let mut tls = tls.borrow_mut(); for _ in 0 .. clocks { tls.step_clocks() } }) }
