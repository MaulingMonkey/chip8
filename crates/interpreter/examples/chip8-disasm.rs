use maulingmonkey_chip8_interpreter::*;



fn main() {
    let mut args = std::env::args_os();
    let _exe = args.next();
    let ch8 = std::path::PathBuf::from(args.next().expect("Usage: chip8-disasm some/rom.ch8"));
    let ch8io = std::fs::File::open(&ch8).unwrap_or_else(|err| panic!("unable to open {}: {err}", ch8.display()));

    let mut memory = Memory4K::new();
    let start = Addr::PROGRAM_START_TYPICAL;
    memory.copy_from_io(start, ch8io).expect("failed to copy ROM into memory");

    let mut addr = 0x200;
    for op in memory.as_words_ref()[start.to_usize()/2..].iter().copied() {
        if op == 0 { break }
        println!("0x{addr:03x}    {:?}", Op(u16::from_be(op)));
        addr += 2;
    }
}
