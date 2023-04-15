use core::fmt::{self, Debug, Display, Formatter};



/// ([u16]) — A CHIP8 address, probably.  As few as 1**2** bits (4K memory) might be used?
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Addr(pub u16);
impl Display for Addr { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "0x{:03x}", self.0) } }
impl Debug   for Addr { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "0x{:03x}", self.0) } }
