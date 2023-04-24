#![cfg(target_arch = "wasm32")]

mod console;
mod wasi_snapshot_preview1;

use maulingmonkey_chip8_interpreter::*;
use core::cell::Cell;



#[derive(Default)] struct Website;
impl Syscalls for Website {
    fn get_key(&self) -> Option<u8> {
        #[link(wasm_import_module = "chip8")] extern "C" { fn get_key() -> u32; }
        unsafe { get_key() }.try_into().ok()
    }

    fn is_pressed(&self, key: u8) -> bool {
        #[link(wasm_import_module = "chip8")] extern "C" { fn is_pressed(key: u32) -> u32; }
        0 != unsafe { is_pressed(key.into()) }
    }

    fn sound_play(&self) {
        #[link(wasm_import_module = "chip8")] extern "C" { fn sound_play(); }
        unsafe { sound_play() }
    }

    fn sound_stop(&self) {
        #[link(wasm_import_module = "chip8")] extern "C" { fn sound_stop(); }
        unsafe { sound_stop() }
    }

    fn render(&self, screen: &ScreenMonochrome64x32) {
        #[link(wasm_import_module = "chip8")] extern "C" { fn render(ptr: *const ScreenMonochrome64x32); }
        unsafe { render(screen) }
    }
}



thread_local! {
    static CONTEXT : Cell<tls::ContextId> = Cell::new(tls::create_context(&include_bytes!("../../../examples/sierpinski.ch8")[..]));
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

    tls::set_syscalls_static(&Website);
    CONTEXT.with(|_|{});
}



#[no_mangle] pub extern "C" fn reset(rom: &[u8; 0xD00]) {
    CONTEXT.with(|ctx| {
        tls::destroy_context(ctx.get());
        ctx.set(tls::create_context(&rom[..]));
    //    let mut tls = tls.borrow_mut();
    //    ctx.registers.sound_playing = tls.registers.sound_playing;
    //    *tls = ctx;
    });
}

#[no_mangle] pub extern "C" fn update() { tls::update() }
