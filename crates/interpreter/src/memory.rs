use crate::*;
use std::io;



/// 4 KiB of (wrapping 12-bit addressed) memory
#[derive(Clone)] pub struct Memory4K([u64; 1<<9]);
impl Default for Memory4K { fn default() -> Self { Self::new() } }

impl Memory4K {
    pub const fn new() -> Self { Self([0; 1<<9]) }

    pub fn read(&self, addr: Addr) -> u8 { self.as_bytes_ref().get(addr.to_usize() & 0xFFF).copied().unwrap_or(0) }
    pub fn read16(&self, addr: Addr) -> u16 { u16::from_be_bytes([self.read(addr), self.read(Addr(addr.0+1))]) }
    pub fn write(&mut self, addr: Addr, value: u8) { self.as_bytes_mut().get_mut(addr.to_usize() & 0xFFF).map(|b| *b = value); }

    pub fn clear(&mut self) { self.0.fill(0) }

    pub fn copy_from_slice(&mut self, addr: Addr, src: &[u8]) -> Result<(), ()> {
        let dst = self.as_bytes_mut().get_mut(addr.to_usize()..).ok_or(())?;
        let dst = dst.get_mut(0..src.len()).ok_or(())?;
        Ok(dst.copy_from_slice(src))
    }

    pub fn copy_from_io(&mut self, addr: Addr, mut src: impl io::Read) -> io::Result<()> {
        const FILE_TOO_LARGE : io::ErrorKind = io::ErrorKind::InvalidData; // FileTooLarge is unstable
        let mut addr = addr.to_usize();
        loop {
            let dst = self.as_bytes_mut().get_mut(addr..).ok_or(FILE_TOO_LARGE)?;
            if dst.is_empty() {
                match src.read(&mut [0u8])? {
                    0 => return Ok(()),
                    _ => Err(FILE_TOO_LARGE)?,
                }
            } else {
                match src.read(dst)? {
                    0 => return Ok(()),
                    n => addr += n,
                }
            }
        }
    }

    pub fn as_bytes_ref(&    self) -> &    [u8; 1<<12] { bytemuck::cast_ref(&    self.0) }
    pub fn as_bytes_mut(&mut self) -> &mut [u8; 1<<12] { bytemuck::cast_mut(&mut self.0) }

    pub fn as_words_ref(&    self) -> &    [u16; 1<<11] { bytemuck::cast_ref(&    self.0) }
    pub fn as_words_mut(&mut self) -> &mut [u16; 1<<11] { bytemuck::cast_mut(&mut self.0) }

    pub fn as_qwords_ref(&    self) -> &    [u64; 1<<9] { &    self.0 }
    pub fn as_qwords_mut(&mut self) -> &mut [u64; 1<<9] { &mut self.0 }

    pub fn screen_monochrome_64x32_mut(&mut self) -> &mut ScreenMonochrome64x32 {
        let screen = &mut self.as_qwords_mut()[(1<<9)-32..];
        let screen = bytemuck::cast_slice_mut(screen);
        let screen = bytemuck::from_bytes_mut(screen);
        screen
    }
}
