use core::convert::TryFrom;
use core::fmt::{self, Debug, Display, Formatter, LowerHex, UpperHex};
use core::num::TryFromIntError;



/// ([u8]) — A 4-bit value between 0 ..= 0xF.  Exhaustive enum under the hood for defaultless `match` statements.
///
/// Constants for pattern matching are `N0` ..= `NF`:
/// ```rust
/// use maulingmonkey_chip8_interpreter::*;
///
/// let n : Nibble = N3;
///
/// match n {
///     N0 => {}, N1 => {}, N2 => {}, N3 => {},
///     N4 => {}, N5 => {}, N6 => {}, N7 => {},
///     N8 => {}, N9 => {}, NA => {}, NB => {},
///     NC => {}, ND => {}, NE => {}, NF => {},
///     // unnecessary:
///     // _ => {}
/// }
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)]
pub struct Nibble(_Nibble);

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(u8)] #[doc(hidden)] enum _Nibble { #[default] _0 = 0, _1, _2, _3, _4, _5, _6, _7, _8, _9, A, B, C, D, E, F }
#[doc(hidden)] pub const N0 : Nibble = Nibble(_Nibble::_0);
#[doc(hidden)] pub const N1 : Nibble = Nibble(_Nibble::_1);
#[doc(hidden)] pub const N2 : Nibble = Nibble(_Nibble::_2);
#[doc(hidden)] pub const N3 : Nibble = Nibble(_Nibble::_3);
#[doc(hidden)] pub const N4 : Nibble = Nibble(_Nibble::_4);
#[doc(hidden)] pub const N5 : Nibble = Nibble(_Nibble::_5);
#[doc(hidden)] pub const N6 : Nibble = Nibble(_Nibble::_6);
#[doc(hidden)] pub const N7 : Nibble = Nibble(_Nibble::_7);
#[doc(hidden)] pub const N8 : Nibble = Nibble(_Nibble::_8);
#[doc(hidden)] pub const N9 : Nibble = Nibble(_Nibble::_9);
#[doc(hidden)] pub const NA : Nibble = Nibble(_Nibble::A);
#[doc(hidden)] pub const NB : Nibble = Nibble(_Nibble::B);
#[doc(hidden)] pub const NC : Nibble = Nibble(_Nibble::C);
#[doc(hidden)] pub const ND : Nibble = Nibble(_Nibble::D);
#[doc(hidden)] pub const NE : Nibble = Nibble(_Nibble::E);
#[doc(hidden)] pub const NF : Nibble = Nibble(_Nibble::F);

impl Nibble {
    pub const fn to_u8(self)    -> u8       { self.0 as _ }
    pub const fn to_u16(self)   -> u16      { self.0 as _ }
    pub const fn to_usize(self) -> usize    { self.0 as _ }

    pub(crate) fn array_ref<T>(self, array: &    [T; 16]) -> &    T { &    array[self.to_usize()] }
    pub(crate) fn array_mut<T>(self, array: &mut [T; 16]) -> &mut T { &mut array[self.to_usize()] }

    #[track_caller] pub const fn truncate16(v: u16) -> Self { Self::truncate8(v as _) }
    #[track_caller] pub const fn truncate8(v: u8) -> Self {
        match v & 0xF {
            0x0 => N0, 0x1 => N1, 0x2 => N2, 0x3 => N3,
            0x4 => N4, 0x5 => N5, 0x6 => N6, 0x7 => N7,
            0x8 => N8, 0x9 => N9, 0xA => NA, 0xB => NB,
            0xC => NC, 0xD => ND, 0xE => NE, 0xF => NF,
            _   => unreachable!(),
        }
    }

    #[track_caller] pub const fn literal(v: u8) -> Self {
        assert!(v < 16, "Nibble::literal(n) is out of bounds (0x0 ..= 0xF)");
        Self::truncate8(v)
    }

    pub fn iter() -> impl Iterator<Item = Self> { (0 .. 16).map(|i| Self::truncate8(i)) }
}

impl From<Nibble> for u8    { fn from(value: Nibble) -> Self { value.to_u8()    } }
impl From<Nibble> for usize { fn from(value: Nibble) -> Self { value.to_usize() } }
impl TryFrom<usize  > for Nibble { type Error = TryFromIntError; fn try_from(value: usize) -> Result<Self, Self::Error> { if value < 16 { Ok(Self::truncate8(value as _)) } else { Err(u8::try_from(-1).unwrap_err()) } } }
impl TryFrom<u64    > for Nibble { type Error = TryFromIntError; fn try_from(value: u64  ) -> Result<Self, Self::Error> { if value < 16 { Ok(Self::truncate8(value as _)) } else { Err(u8::try_from(-1).unwrap_err()) } } }
impl TryFrom<u32    > for Nibble { type Error = TryFromIntError; fn try_from(value: u32  ) -> Result<Self, Self::Error> { if value < 16 { Ok(Self::truncate8(value as _)) } else { Err(u8::try_from(-1).unwrap_err()) } } }
impl TryFrom<u16    > for Nibble { type Error = TryFromIntError; fn try_from(value: u16  ) -> Result<Self, Self::Error> { if value < 16 { Ok(Self::truncate8(value as _)) } else { Err(u8::try_from(-1).unwrap_err()) } } }
impl TryFrom<u8     > for Nibble { type Error = TryFromIntError; fn try_from(value: u8   ) -> Result<Self, Self::Error> { if value < 16 { Ok(Self::truncate8(value as _)) } else { Err(u8::try_from(-1).unwrap_err()) } } }

impl Debug      for Nibble { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug      ::fmt(&self.to_u8(), fmt) } }
impl Display    for Nibble { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display    ::fmt(&self.to_u8(), fmt) } }
impl LowerHex   for Nibble { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { LowerHex   ::fmt(&self.to_u8(), fmt) } }
impl UpperHex   for Nibble { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { UpperHex   ::fmt(&self.to_u8(), fmt) } }
