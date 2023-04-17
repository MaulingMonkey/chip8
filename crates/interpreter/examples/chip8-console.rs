use maulingmonkey_chip8_interpreter::*;

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::time::{Duration, Instant};



static SOUND : AtomicBool = AtomicBool::new(false); // XXX: consider a channel instead
struct Console;
impl Syscalls for Console {
    fn get_key(&self) -> Option<u8> { None }
    fn is_pressed(&self, _key: u8) -> bool { false }
    fn sound_play(&self) { SOUND.store(true, Relaxed) }
    fn sound_stop(&self) { SOUND.store(false, Relaxed) }
}

fn main() {
    #[cfg(windows)] enable_virtual_terminal_sequences();

    let mut args = std::env::args_os();
    let _exe = args.next();
    let ch8 = std::path::PathBuf::from(args.next().expect("Usage: chip8-console some/rom.ch8"));
    let ch8io = std::fs::File::open(&ch8).unwrap_or_else(|err| panic!("unable to open {}: {err}", ch8.display()));

    #[cfg(windows)] if let Err(err) = std::thread::Builder::new().name("sound thread".into()).spawn(|| sound_thread()) {
        eprintln!("warning: failed to spawn sound thread: {err:?}");
    }

    let mut ctx = Context {
        memory:     Default::default(),
        registers:  Default::default(),
        syscalls:   Console,
    };
    ctx.registers.pc = Addr::PROGRAM_START_TYPICAL;
    ctx.memory.copy_from_io(ctx.registers.pc, ch8io).expect("failed to copy ROM into memory");
    ctx.memory.copy_from_slice(Addr::SYSTEM_INTERPRETER_FONTS_START, bytemuck::cast_slice(font::DEFAULT)).expect("failed to copy font into memory"); // ≈ pointless?

    let start = Instant::now();
    let mut next_redraw = start;
    let mut next_step = start;
    loop {
        let now = Instant::now();

        // Step logic
        while now >= next_step {
            ctx.try_step_many(500/60); // aim for 500 hz
            ctx.step_clocks();
            next_step += Duration::from_millis(1000/60);
        }

        // Redraw
        if now >= next_redraw {
            next_redraw = now + Duration::from_millis(200);
            let _ = std::io::stdout().write_all(b"\x1b[H"); // return cursor to 0,0
            let screen = ctx.screen();
            let mut stdout = std::io::stdout().lock();
            for y in 0 .. 32 {
                for x in 0 .. 64 {
                    let ch = [" ", "█"][usize::from(screen.get_pixel(x, y))];
                    let _ = stdout.write_all(ch.as_bytes());
                }
                let _ = stdout.write_all(b"\r\n");
            }
        }
    }
}

/// <https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences>
#[cfg(windows)] fn enable_virtual_terminal_sequences() {
    use winapi::um::{
        consoleapi::*,
        processenv::GetStdHandle,
        winbase::STD_OUTPUT_HANDLE,
        wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    };

    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    assert!(!stdout.is_null(), "GetStdHandle(STD_OUTPUT_HANDLE) failed, cannot enable virtual terminal sequences for clearing the screen");

    let mut mode = 0;
    assert!(0 != unsafe { GetConsoleMode(stdout, &mut mode) }); // TODO: GetLastError
    mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    assert!(0 != unsafe { SetConsoleMode(stdout, mode) }); // TODO: GetLastError
}

/// Offloading XAudio2 use to another thread is overkill, but nicely isolates MTA init on the off chance that's important.
#[cfg(windows)] fn sound_thread() {
    use thindx_xaudio2::xaudio2_9::*;

    mcom::init::mta().expect("MTA required for xaudio2");
    unsafe { thindx_xaudio2::disable_catch_unwind() };
    let xaudio2 = unsafe { xaudio2::create(None, None) }.expect("xaudio2::create");
    let _master = xaudio2.create_mastering_voice(xaudio2::DEFAULT_CHANNELS, xaudio2::DEFAULT_SAMPLERATE, 0, (), None, xaudio2::DEFAULT_AUDIO_CATEGORY).expect("create_mastering_voice");
    let hz = 44100;
    let samples = hz / 440; // https://en.wikipedia.org/wiki/A440_(pitch_standard)
    let format = xaudio2::TypedSourceFormat::pcm(hz);
    let callback = xaudio2::VoiceCallbackWrapper::new(VoiceCallback);
    let beep = xaudio2.create_source_voice_typed_callback(&format, 0, xaudio2::DEFAULT_FREQ_RATIO, &callback, None /* defaults to master */, None).expect("create beep");
    let samples = (0 .. samples).map(|s| { let s = f32::sin((s as f32) * 2.0 * std::f32::consts::PI / (samples as f32)); [s, s]}).collect::<Vec<_>>();
    beep.set_volume(0.0, xaudio2::COMMIT_NOW).expect("beep.set_volume(0.0) (init)");
    beep.submit_source_buffer(xaudio2::END_OF_STREAM, samples, .., .., xaudio2::LOOP_INFINITE, ()).expect("beep.submit_source_buffer");
    beep.start(0, xaudio2::COMMIT_NOW).expect("beep.start()");

    let mut prev = false;
    loop {
        let now = SOUND.load(Relaxed);
        if prev == now { continue }
        beep.set_volume(if now { 0.2 } else { 0.0 }, xaudio2::COMMIT_NOW).expect("beep.set_volume(SOUND ? 0.2 : 0.0)");
        prev = now;
        std::thread::yield_now();
    }

    struct VoiceCallback;
    impl xaudio2::VoiceCallback for VoiceCallback {
        type BufferContext = ();
        // As expected, these callbacks all fire on an XAudio2 thread:
        fn on_voice_processing_pass_start(&self, _bytes_required: u32) {}
        fn on_voice_processing_pass_end(&self) {}
        fn on_loop_end(&self, _buffer_context: &Self::BufferContext) {}
        fn on_buffer_start(&self, _buffer_context: &Self::BufferContext) {}
        fn on_buffer_end(&self, _buffer_context: Self::BufferContext) {}
        fn on_stream_end(&self) {}
        fn on_voice_error(&self, _buffer_context: &Self::BufferContext, error: winresult::HResult) { panic!("{error:?}"); }
    }
}
