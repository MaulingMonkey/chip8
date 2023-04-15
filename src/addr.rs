use core::fmt::{self, Debug, Display, Formatter};



/// ([u16]) — A CHIP8 address, probably.  As few as 1**2** bits (4K memory) might be used?
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Addr(pub u16);
impl Display for Addr { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "0x{:03x}", self.0) } }
impl Debug   for Addr { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "0x{:03x}", self.0) } }

impl Addr {
    pub const fn to_u16(self)   -> u16      { self.0 as _ }
    pub const fn to_usize(self) -> usize    { self.0 as _ }

    // https://en.wikipedia.org/wiki/CHIP-8#Memory
    // https://tonisagrista.com/blog/2021/chip8-spec/

    pub const SYSTEM_INTERPRETER_FONTS_START    : Addr = Addr(0x000);
    pub const SYSTEM_INTERPRETER_FONTS_END      : Addr = Addr(0x200);

    pub const PROGRAM_START_TYPICAL             : Addr = Addr(0x200);
    pub const PROGRAM_START_ETI_660             : Addr = Addr(0x600);

    pub const SYSTEM_STACK_ETC_START            : Addr = Addr(0xEA0); // EA0 ..= EFF =  96 bytes (12+ 16-bit stack = 48+ bytes)
    pub const SYSTEM_DISPLAY_START              : Addr = Addr(0xF00); // F00 ..= FFF = 256 bytes = 64*32 bits = display resolution
}
