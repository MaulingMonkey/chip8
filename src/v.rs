use crate::*;
use core::fmt::{self, Debug, Display, Formatter};



/// ([Nibble]) — One of the general purpouse registers, `V0` ..= `VF`
///
/// Other registers this cannot reference include:
/// *   `I`     (general data address register)
/// *   `PC`    (program counter)
/// *   `SP`    (internal return stack pointer)
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct V(pub Nibble);
impl Display for V { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "V{:X}", self.0) } }
impl Debug   for V { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "V{:X}", self.0) } }

#[doc(hidden)] pub const V0 : V = V(N0);
#[doc(hidden)] pub const VF : V = V(NF);
