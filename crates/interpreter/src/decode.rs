use crate::*;

/// An [Op]::[decode](Op::decode) visitor for decoding [u16]s to various specific instructions.
pub trait Decode {
    /// Returned by all fns, and eventually [`Op::decode`].
    type Result;

    // https://en.wikipedia.org/wiki/CHIP-8#Opcode_table

    /// Various bit patterns not covered by another fn.
    fn invalid                                  (&mut self, op: u16)                    -> Self::Result;
    #[doc = "`0NNN`"] fn call_mcs               (&mut self, addr: Addr)                 -> Self::Result;
    #[doc = "`00E0`"] fn display_clear          (&mut self)                             -> Self::Result { self.call_mcs(Addr(0x00E0)) }
    #[doc = "`00EE`"] fn flow_return            (&mut self)                             -> Self::Result { self.call_mcs(Addr(0x00EE)) }
    #[doc = "`1NNN`"] fn flow_goto              (&mut self, addr: Addr)                 -> Self::Result;
    #[doc = "`2NNN`"] fn flow_call              (&mut self, addr: Addr)                 -> Self::Result;
    #[doc = "`3XNN`"] fn skip_if_v_eq_c         (&mut self, v: V, c: u8)                -> Self::Result;
    #[doc = "`4XNN`"] fn skip_if_v_ne_c         (&mut self, v: V, c: u8)                -> Self::Result;
    #[doc = "`5XY0`"] fn skip_if_v_eq_v         (&mut self, vx: V, vy: V)               -> Self::Result;
    #[doc = "`6XNN`"] fn set_v_c                (&mut self, vx: V, c: u8)               -> Self::Result;
    #[doc = "`7XNN`"] fn add_v_c                (&mut self, vx: V, c: u8)               -> Self::Result;
    #[doc = "`8XY0`"] fn set_v_v                (&mut self, vx: V, vy: V)               -> Self::Result;
    #[doc = "`8XY1`"] fn bitor_v_v              (&mut self, vx: V, vy: V)               -> Self::Result;
    #[doc = "`8XY2`"] fn bitand_v_v             (&mut self, vx: V, vy: V)               -> Self::Result;
    #[doc = "`8XY3`"] fn bitxor_v_v             (&mut self, vx: V, vy: V)               -> Self::Result;
    #[doc = "`8XY4`"] fn add_v_v                (&mut self, vx: V, vy: V)               -> Self::Result;
    #[doc = "`8XY5`"] fn sub_v_v                (&mut self, vx: V, vy: V)               -> Self::Result;
    #[doc = "`8XY6`"] fn shr1_v                 (&mut self, vx: V, _y: V)               -> Self::Result; // is `y` unused?
    #[doc = "`8XY7`"] fn sub_v_v_alt            (&mut self, vx: V, vy: V)               -> Self::Result;
    #[doc = "`8XYE`"] fn shl1_v                 (&mut self, vx: V, _y: V)               -> Self::Result; // is `y` unused?
    #[doc = "`9XY0`"] fn skip_if_v_ne_v         (&mut self, vx: V, vy: V)               -> Self::Result;
    #[doc = "`ANNN`"] fn set_i_c                (&mut self, c: Addr)                    -> Self::Result;
    #[doc = "`BNNN`"] fn set_pc_v0_plus_c       (&mut self, _v0: (), c: Addr)           -> Self::Result;
    #[doc = "`CXNN`"] fn set_v_rand_mask        (&mut self, v: V, mask: u8)             -> Self::Result;
    #[doc = "`DXYN`"] fn draw_x_y_h             (&mut self, vx: V, vy: V, h: Nibble)    -> Self::Result;
    #[doc = "`EX9E`"] fn skip_if_pressed        (&mut self, key: V)                     -> Self::Result;
    #[doc = "`EXA1`"] fn skip_unless_pressed    (&mut self, key: V)                     -> Self::Result;
    #[doc = "`FX07`"] fn get_delay_timer        (&mut self, v: V)                       -> Self::Result;
    #[doc = "`FX0A`"] fn await_key              (&mut self, v: V)                       -> Self::Result;
    #[doc = "`FX15`"] fn set_delay_timer        (&mut self, v: V)                       -> Self::Result;
    #[doc = "`FX18`"] fn set_sound_timer        (&mut self, v: V)                       -> Self::Result;
    #[doc = "`FX1E`"] fn add_i_v                (&mut self, v: V)                       -> Self::Result;
    #[doc = "`FX29`"] fn set_i_sprite           (&mut self, v: V)                       -> Self::Result;
    #[doc = "`FX33`"] fn set_i_bcd              (&mut self, v: V)                       -> Self::Result;
    #[doc = "`FX55`"] fn reg_dump               (&mut self, v: V)                       -> Self::Result;
    #[doc = "`FX65`"] fn reg_load               (&mut self, v: V)                       -> Self::Result;
}

impl Op {
    /// `self` → [`Decode`] → [`Decode::Result`]
    pub fn decode<D: Decode>(self, decode: &mut D) -> D::Result {
        const fn addr3(v: u16) -> Addr { Addr(v & 0xFFF) }
        const fn n(v: u16) -> Nibble { Nibble::truncate16(v) }
        const fn v(v: u16) -> V { V(n(v)) }
        const fn b(v: u16) -> u8 { v as _ }

        let op = self.0;

        match n(op>>12) {
            N0 => match op {
                0x00E0  => decode.display_clear(),
                0x00EE  => decode.flow_return(),
                other   => decode.call_mcs(Addr(other)),
            },
            N1 => decode.flow_goto(addr3(op)),
            N2 => decode.flow_call(addr3(op)),
            N3 => decode.skip_if_v_eq_c(v(op>>8), b(op>>0)),
            N4 => decode.skip_if_v_ne_c(v(op>>8), b(op>>0)),
            N5 if n(op>>0) == N0 => decode.skip_if_v_eq_v(v(op>>8), v(op>>4)),
            N5 => decode.invalid(op),
            N6 => decode.set_v_c(v(op>>8), b(op>>0)),
            N7 => decode.add_v_c(v(op>>8), b(op>>0)),
            N8 => {
                let vx = v(op>>8);
                let vy = v(op>>4);
                match n(op>>0) {
                    N0 => decode.set_v_v        (vx, vy),
                    N1 => decode.bitor_v_v      (vx, vy),
                    N2 => decode.bitand_v_v     (vx, vy),
                    N3 => decode.bitxor_v_v     (vx, vy),
                    N4 => decode.add_v_v        (vx, vy),
                    N5 => decode.sub_v_v        (vx, vy),
                    N6 => decode.shr1_v         (vx, vy),
                    N7 => decode.sub_v_v_alt    (vx, vy),
                    NE => decode.shl1_v         (vx, vy),
                    _  => decode.invalid(op),
                    // unused: 8 ..= D, F
                }
            },
            N9 => {
                let vx = v(op>>8);
                let vy = v(op>>4);
                match n(op>>0) {
                    N0 => decode.skip_if_v_ne_v(vx, vy),
                    _ => decode.invalid(op),
                }
            },
            NA => decode.set_i_c(addr3(op)),
            NB => decode.set_pc_v0_plus_c((), addr3(op>>0)),
            NC => decode.set_v_rand_mask(v(op>>8), b(op>>0)),
            ND => decode.draw_x_y_h(v(op>>8), v(op>>4), n(op>>0)),
            NE => {
                let vx = v(op>>8);
                let c  = b(op>>0);
                match c {
                    0x9E => decode.skip_if_pressed      (vx),
                    0xA1 => decode.skip_unless_pressed  (vx),
                    _    => decode.invalid(op),
                }
            },
            NF => {
                let vx = v(op>>8);
                let c  = b(op>>0);
                match c {
                    0x07 => decode.get_delay_timer  (vx),
                    0x0A => decode.await_key        (vx),
                    0x15 => decode.set_delay_timer  (vx),
                    0x18 => decode.set_sound_timer  (vx),
                    0x1E => decode.add_i_v          (vx),
                    0x29 => decode.set_i_sprite     (vx),
                    0x33 => decode.set_i_bcd        (vx),
                    0x55 => decode.reg_dump         (vx),
                    0x65 => decode.reg_load         (vx),
                    _    => decode.invalid(op),
                }
            },
        }
    }
}
