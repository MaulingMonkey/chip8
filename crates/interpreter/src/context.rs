use crate::*;



/// Execution context with methods like [`try_step_single`](Self::try_step_single), [`try_step_many`](Self::try_step_many), etc.
#[derive(Default)] pub struct Context<S: Syscalls> {
    pub registers:  Registers,
    pub memory:     Memory4K,
    pub syscalls:   S,
    // ...?
}

impl<S: Syscalls> core::fmt::Debug for Context<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Context {{ ... }}") }
}

impl<S: Syscalls> Context<S> {
    pub fn new() -> Self where S : Default { Self::default() }

    pub fn screen(&mut self) -> &mut ScreenMonochrome64x32 { self.memory.screen_monochrome_64x32_mut() }

    /// Try to run a single [`Op`]/instruction.  Returns `true` if successful.
    pub fn try_step_single(&mut self) -> bool {
        let op = Op(self.memory.read16(self.registers.pc));
        return op.decode(&mut Step(self));

        #[repr(transparent)] struct Step<'a, S: Syscalls>(&'a mut Context<S>);
        impl<S: Syscalls> Decode for Step<'_, S> {
            type Result = bool;
            #[inline(always)] fn invalid                (&mut self, op: u16)                    -> Self::Result { panic!("invalid instruction: 0x{op:04x} @ {}", self.0.registers.pc) }
            #[inline(always)] fn call_mcs               (&mut self, addr: Addr)                 -> Self::Result { panic!("invalid mcs call: {addr} @ {}", self.0.registers.pc) }
            #[inline(always)] fn display_clear          (&mut self)                             -> Self::Result { self.0.screen().clear(); self.0.step() }
            #[inline(always)] fn flow_return            (&mut self)                             -> Self::Result { self.0.registers.pc = self.0.registers.stack.pop().expect("return without any stack"); true }
            #[inline(always)] fn flow_goto              (&mut self, addr: Addr)                 -> Self::Result { self.0.registers.pc = addr; true }
            #[inline(always)] fn flow_call              (&mut self, addr: Addr)                 -> Self::Result { self.0.registers.stack.push(Addr(self.0.registers.pc.0 + 2)); self.0.registers.pc = addr; true }
            #[inline(always)] fn skip_if_v_eq_c         (&mut self, v: V, c: u8)                -> Self::Result { self.0.step_skip_if(self.0.registers[v] == c) }
            #[inline(always)] fn skip_if_v_ne_c         (&mut self, v: V, c: u8)                -> Self::Result { self.0.step_skip_if(self.0.registers[v] != c) }
            #[inline(always)] fn skip_if_v_eq_v         (&mut self, vx: V, vy: V)               -> Self::Result { self.0.step_skip_if(self.0.registers[vx] == self.0.registers[vy]) }
            #[inline(always)] fn set_v_c                (&mut self, vx: V, c: u8)               -> Self::Result { self.0.registers[vx] =  c; self.0.step() }
            #[inline(always)] fn add_v_c                (&mut self, vx: V, c: u8)               -> Self::Result { self.0.registers[vx] =  self.0.registers[vx].wrapping_add(c); self.0.step() }
            #[inline(always)] fn set_v_v                (&mut self, vx: V, vy: V)               -> Self::Result { self.0.registers[vx] =  self.0.registers[vy]; self.0.step() }
            #[inline(always)] fn bitor_v_v              (&mut self, vx: V, vy: V)               -> Self::Result { self.0.registers[vx] |= self.0.registers[vy]; self.0.step() }
            #[inline(always)] fn bitand_v_v             (&mut self, vx: V, vy: V)               -> Self::Result { self.0.registers[vx] &= self.0.registers[vy]; self.0.step() }
            #[inline(always)] fn bitxor_v_v             (&mut self, vx: V, vy: V)               -> Self::Result { self.0.registers[vx] ^= self.0.registers[vy]; self.0.step() }
            #[inline(always)] fn add_v_v                (&mut self, vx: V, vy: V)               -> Self::Result { let (x, y) = (self.0.registers[vx], self.0.registers[vy]); self.0.registers[VF] = x.checked_add(y).is_none().into(); self.0.registers[vx] = x.wrapping_add(y); self.0.step() }
            #[inline(always)] fn sub_v_v                (&mut self, vx: V, vy: V)               -> Self::Result { let (x, y) = (self.0.registers[vx], self.0.registers[vy]); self.0.registers[VF] = x.checked_sub(y).is_none().into(); self.0.registers[vx] = x.wrapping_sub(y); self.0.step() }
            #[inline(always)] fn shr1_v                 (&mut self, vx: V, vy: V)               -> Self::Result { let y = self.0.registers[vy]; let carry = y & 0x01; self.0.registers[vx] = y >> 1; self.0.registers[VF] = carry; self.0.step() }
            #[inline(always)] fn sub_v_v_alt            (&mut self, vx: V, vy: V)               -> Self::Result { let (x, y) = (self.0.registers[vx], self.0.registers[vy]); self.0.registers[VF] = y.checked_sub(x).is_none().into(); self.0.registers[vx] = y.wrapping_sub(x); self.0.step() }
            #[inline(always)] fn shl1_v                 (&mut self, vx: V, vy: V)               -> Self::Result { let y = self.0.registers[vy]; let carry = y & 0x80; self.0.registers[vx] = y << 1; self.0.registers[VF] = carry; self.0.step() }
            #[inline(always)] fn skip_if_v_ne_v         (&mut self, vx: V, vy: V)               -> Self::Result { self.0.step_skip_if(self.0.registers[vx] != self.0.registers[vy]) }
            #[inline(always)] fn set_i_c                (&mut self, c: Addr)                    -> Self::Result { self.0.registers.i = c; self.0.step() }
            #[inline(always)] fn set_pc_v0_plus_c       (&mut self, _v0: (), c: Addr)           -> Self::Result { self.0.registers.pc = Addr((u16::from(self.0.registers[V0]) + c.0) & 0xFFF); true } // XXX: overflow?
            #[inline(always)] fn set_v_rand_mask        (&mut self, v: V, mask: u8)             -> Self::Result { self.0.registers[v] = self.0.syscalls.rand() & mask; self.0.step() }

            #[inline(always)] fn draw_x_y_h(&mut self, vx: V, vy: V, h: Nibble) -> Self::Result {
                let x = self.0.registers[vx];
                let y = self.0.registers[vy];
                let h = h.to_usize();
                let mut sprite = [0u8; 16];
                let sprite = &mut sprite[..h];
                sprite.copy_from_slice(&self.0.memory.as_bytes_ref()[self.0.registers.i.to_usize()..][..h]);
                let overlap = self.0.screen().draw_sprite(x.into(), y.into(), sprite);
                self.0.registers[VF] = overlap.into();
                self.0.step()
            }

            #[inline(always)] fn skip_if_pressed        (&mut self, key: V)                     -> Self::Result { self.0.step_skip_if(self.0.syscalls.is_pressed(self.0.registers[key])) }
            #[inline(always)] fn skip_unless_pressed    (&mut self, key: V)                     -> Self::Result { self.0.step_skip_if(self.0.syscalls.is_pressed(self.0.registers[key])) }
            #[inline(always)] fn get_delay_timer        (&mut self, v: V)                       -> Self::Result { self.0.registers[v] = self.0.registers.delay_timer; self.0.step() }
            #[inline(always)] fn await_key              (&mut self, v: V)                       -> Self::Result { let Some(key) = self.0.syscalls.get_key() else { return false }; self.0.registers[v] = key; self.0.step() }
            #[inline(always)] fn set_delay_timer        (&mut self, v: V)                       -> Self::Result { self.0.registers.delay_timer = self.0.registers[v]; self.0.step() }
            #[inline(always)] fn set_sound_timer        (&mut self, v: V)                       -> Self::Result { self.0.registers.sound_timer = self.0.registers[v]; self.0.step() }
            #[inline(always)] fn add_i_v                (&mut self, v: V)                       -> Self::Result { self.0.registers.i.0 += u16::from(self.0.registers[v]); self.0.step() }
            #[inline(always)] fn set_i_sprite           (&mut self, v: V)                       -> Self::Result { self.0.registers.i.0 = Addr::SYSTEM_INTERPRETER_FONTS_START.0 + u16::from(self.0.registers[v]) * 5; self.0.step() }
            #[inline(always)] fn set_i_bcd              (&mut self, v: V)                       -> Self::Result { self.0.memory.copy_from_slice(self.0.registers.i, &bcd(self.0.registers[v])).map_or(false, |_| self.0.step()) }
            #[inline(always)] fn reg_dump               (&mut self, v: V)                       -> Self::Result { for v in V::iter().take(v.0.to_usize()+1) { self.0.memory.write(Addr(self.0.registers.i.0 + v.0.to_u16()), self.0.registers[v]) } self.0.step() }
            #[inline(always)] fn reg_load               (&mut self, v: V)                       -> Self::Result { for v in V::iter().take(v.0.to_usize()+1) { self.0.registers[v] = self.0.memory.read(Addr(self.0.registers.i.0 + v.0.to_u16())) } self.0.step() }
        }
    }

    /// Try to run `steps` instructions.  Returns the number of instructions actually executed (may be 0).
    pub fn try_step_many(&mut self, steps: usize) -> usize {
        for step in 0 .. steps {
            if !self.try_step_single() { return step }
        }
        steps
    }

    pub fn step_clocks(&mut self) {
        self.registers.delay_timer = self.registers.delay_timer.saturating_sub(1);
        self.registers.sound_timer = self.registers.sound_timer.saturating_sub(1);

        // N.B. by waiting until after the `saturating_sub`s to check timer state, sound will only play if sound_timer >= 2.
        // This is intentional - as noted by https://github.com/mattmikolay/chip-8/wiki/Mastering-CHIP%E2%80%908#timers :
        //
        // "It should be noted that in the COSMAC VIP manual, it was made clear that the minimum value that the timer
        // will respond to is 02. Thus, setting the timer to a value of 01 would have no audible effect."
        //
        // Other impls might have different behavior, but I've chosen to match the COSMAC VIP in this regard.
        // https://en.wikipedia.org/wiki/COSMAC_VIP
        //
        let should_play = self.registers.sound_timer > 0;
        match (self.registers.sound_playing, should_play) {
            (true, false)   => self.syscalls.sound_stop(),
            (false, true)   => self.syscalls.sound_play(),
            _               => {},
        }
        self.registers.sound_playing = should_play;
    }

    #[inline] fn advance(&mut self, n: u16) -> bool { self.registers.pc.0 += n; true }
    fn step(&mut self) -> bool { self.advance(2) }
    fn step_skip_if(&mut self, skip: bool) -> bool { self.advance(if skip { 4 } else { 2 }) }
}

fn bcd(b: u8) -> [u8; 3] { [b / 100, b/10%10, b%10] }
#[test] fn test_bcd() { assert_eq!([1, 2, 3], bcd(123)) }
