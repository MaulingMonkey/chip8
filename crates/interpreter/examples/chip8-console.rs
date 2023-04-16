use maulingmonkey_chip8_interpreter::*;
type Context = maulingmonkey_chip8_interpreter::Context<()>;

use std::io::Write;
use std::time::{Duration, Instant};



fn main() {
    #[cfg(windows)] enable_virtual_terminal_sequences();

    let mut args = std::env::args_os();
    let _exe = args.next();
    let ch8 = std::path::PathBuf::from(args.next().expect("Usage: chip8-console some/rom.ch8"));
    let ch8io = std::fs::File::open(&ch8).unwrap_or_else(|err| panic!("unable to open {}: {err}", ch8.display()));

    let mut ctx = Context::default();
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
