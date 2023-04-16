use crate::*;



/// V0 ..= VF, I, PC, and Stack
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Registers { // https://en.wikipedia.org/wiki/CHIP-8#Registers
    /// General purpouse registers
    pub v:      [u8; 16],

    /// Address register
    pub i:      Addr,

    /// Program Counter
    pub pc:     Addr,

    pub(crate) stack:       Vec<Addr>, // could also live somewhere in memory[0xEA0 ..= 0xEFF]
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,
    pub(crate) sound_playing: bool,
}

impl core::ops::Index<V> for Registers {
    type Output = u8;
    fn index(&self, index: V) -> &Self::Output { index.0.array_ref(&self.v) }
}

impl core::ops::IndexMut<V> for Registers {
    fn index_mut(&mut self, index: V) -> &mut Self::Output { index.0.array_mut(&mut self.v) }
}
