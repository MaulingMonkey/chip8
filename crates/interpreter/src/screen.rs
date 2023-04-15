use bytemuck::*;

/// 64 x 32 x 1 bit per pixel = 256 bytes = 32 qwords
#[derive(Clone, Copy, Zeroable, Pod)] #[repr(transparent)] pub struct ScreenMonochrome64x32([u64; 32]);
impl Default for ScreenMonochrome64x32 { fn default() -> Self { Self::new() } }

impl ScreenMonochrome64x32 {
    pub const fn new() -> Self { Self([0; 32]) }
    pub const WIDTH     : usize = 64;
    pub const HEIGHT    : usize = 32;

    pub fn clear(&mut self) {
        self.0.fill(0)
    }

    pub fn try_get_pixel(&self, x: usize, y: usize) -> Option<bool> {
        let row = u64::from_be(*self.0.get(y)?);
        let mask = 1u64.checked_shl(x.try_into().ok()?)?;
        Some(row & mask != 0)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        let pixel = self.try_get_pixel(x, y);
        debug_assert!(pixel.is_some(), "get_pixel({x}, {y}) out of bounds");
        pixel.unwrap_or(false)
    }

    pub fn try_set_pixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), ()> {
        let mask = 1u64.checked_shl(x.try_into().map_err(|_| ())?).ok_or(())?.to_be();
        let row = self.0.get_mut(y).ok_or(())?;
        if value {
            *row |= mask;
        } else {
            *row &=!mask;
        }
        Ok(())
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        let _ = self.try_set_pixel(x, y, value);
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) {
        for (oy, row) in sprite.iter().copied().enumerate() {
            for ox in 0 .. 8 {
                let x = x + ox;
                let y = y + oy;
                if let Some(original) = self.try_get_pixel(x, y) {
                    if row & (0x80 >> ox) != 0 { // left to right
                        self.set_pixel(x, y, !original); // XOR behavior
                    }
                }
            }
        }
    }
}
