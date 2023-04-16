use crate::*;
use core::fmt::{self, Debug, Formatter};



/// ([u16]) — A 16-bit CHIP8 instruction.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct Op(pub u16);

impl Debug for Op {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        return self.decode(&mut DebugOp(fmt));
        struct DebugOp<'a, 'b>(&'a mut Formatter<'b>);
        impl crate::decode::Decode for DebugOp<'_, '_> {
            type Result = fmt::Result;

            fn invalid              (&mut self, op: u16)                    -> Self::Result { write!(self.0, "invalid ; 0x{op:04X}") }
            fn call_mcs             (&mut self, addr: Addr)                 -> Self::Result { write!(self.0, "call_mcs {addr}") }
            fn display_clear        (&mut self)                             -> Self::Result { write!(self.0, "display_clear") }
            fn flow_return          (&mut self)                             -> Self::Result { write!(self.0, "return") }
            fn flow_goto            (&mut self, addr: Addr)                 -> Self::Result { write!(self.0, "pc <- {addr}") }
            fn flow_call            (&mut self, addr: Addr)                 -> Self::Result { write!(self.0, "call {addr}") }
            fn skip_if_v_eq_c       (&mut self, v: V, c: u8)                -> Self::Result { write!(self.0, "skip_if {v} == 0x{c:02X}") }
            fn skip_if_v_ne_c       (&mut self, v: V, c: u8)                -> Self::Result { write!(self.0, "skip_if {v} != 0x{c:02X}") }
            fn skip_if_v_eq_v       (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "skip_if {vx} == {vy}") }
            fn set_v_c              (&mut self, vx: V, c: u8)               -> Self::Result { write!(self.0, "{vx} <- 0x{c:02X}") }
            fn add_v_c              (&mut self, vx: V, c: u8)               -> Self::Result { write!(self.0, "{vx} <- {vx} + 0x{c:02X}") }
            fn set_v_v              (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "{vx} <- {vy}") }
            fn bitor_v_v            (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "{vx} <- {vx} | {vy}") }
            fn bitand_v_v           (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "{vx} <- {vx} & {vy}") }
            fn bitxor_v_v           (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "{vx} <- {vx} ^ {vy}") }
            fn add_v_v              (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "{vx} <- {vx} + {vy}") }
            fn sub_v_v              (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "{vx} <- {vx} - {vy}") }
            fn shr1_v               (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "{vx} <- {vx} >> 1 ; {vy}") }
            fn sub_v_v_alt          (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "{vx} <- {vy} - {vx}") }
            fn shl1_v               (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "{vx} <- {vx} << 1 ; {vy}") }
            fn skip_if_v_ne_v       (&mut self, vx: V, vy: V)               -> Self::Result { write!(self.0, "skip_if {vx} != {vy}") }
            fn set_i_c              (&mut self, c: Addr)                    -> Self::Result { write!(self.0, "i <- {c}") }
            fn set_pc_v0_plus_c     (&mut self, _v0: (), c: Addr)           -> Self::Result { write!(self.0, "pc <- V0 + {c}") }
            fn set_v_rand_mask      (&mut self, v: V, mask: u8)             -> Self::Result { write!(self.0, "{v} <- rand() & 0x{mask:02X}") }
            fn draw_x_y_h           (&mut self, vx: V, vy: V, h: Nibble)    -> Self::Result { write!(self.0, "draw_sprite(x={vx}, y={vy}, h={h}, sprite=i)") }
            fn skip_if_pressed      (&mut self, key: V)                     -> Self::Result { write!(self.0, "skip_if key_pressed({key})") }
            fn skip_unless_pressed  (&mut self, key: V)                     -> Self::Result { write!(self.0, "skip_if !key_pressed({key})") }
            fn get_delay_timer      (&mut self, v: V)                       -> Self::Result { write!(self.0, "{v} <- delay_timer") }
            fn await_key            (&mut self, v: V)                       -> Self::Result { write!(self.0, "{v} <- get_key()") }
            fn set_delay_timer      (&mut self, v: V)                       -> Self::Result { write!(self.0, "delay_timer <- {v}") }
            fn set_sound_timer      (&mut self, v: V)                       -> Self::Result { write!(self.0, "sound_timer <- {v}") }
            fn add_i_v              (&mut self, v: V)                       -> Self::Result { write!(self.0, "i <- i + {v}") }
            fn set_i_sprite         (&mut self, v: V)                       -> Self::Result { write!(self.0, "i <- sprites[{v}]") }
            fn set_i_bcd            (&mut self, v: V)                       -> Self::Result { write!(self.0, "i[0..3] <- bcd({v})") }
            fn reg_dump             (&mut self, v: V)                       -> Self::Result { write!(self.0, "i[0..={n}] <- [V0..={v}]", n = v.0.to_u8()) }
            fn reg_load             (&mut self, v: V)                       -> Self::Result { write!(self.0, "[V0..={v}] <- i[0..={n}]", n = v.0.to_u8()) }
        }
    }
}

#[test] fn op_debug() {
    assert_eq!(format!("{:?}", Op(0x0123)), "call_mcs 0x123");
    assert_eq!(format!("{:?}", Op(0x00E0)), "display_clear");
    assert_eq!(format!("{:?}", Op(0x00EE)), "return");

    assert_eq!(format!("{:?}", Op(0x1234)), "pc <- 0x234");
    assert_eq!(format!("{:?}", Op(0x2345)), "call 0x345");
    assert_eq!(format!("{:?}", Op(0x3456)), "skip_if V4 == 0x56"); // TODO: if → unless ?
    assert_eq!(format!("{:?}", Op(0x4567)), "skip_if V5 != 0x67"); // TODO: if → unless ?

    assert_eq!(format!("{:?}", Op(0x5670)), "skip_if V6 == V7");
    assert_eq!(format!("{:?}", Op(0x5678)), "invalid ; 0x5678");

    assert_eq!(format!("{:?}", Op(0x6123)), "V1 <- 0x23");

    assert_eq!(format!("{:?}", Op(0x7234)), "V2 <- V2 + 0x34");

    assert_eq!(format!("{:?}", Op(0x8210)), "V2 <- V1");
    assert_eq!(format!("{:?}", Op(0x8321)), "V3 <- V3 | V2");
    assert_eq!(format!("{:?}", Op(0x8432)), "V4 <- V4 & V3");
    assert_eq!(format!("{:?}", Op(0x8543)), "V5 <- V5 ^ V4");
    assert_eq!(format!("{:?}", Op(0x8654)), "V6 <- V6 + V5");
    assert_eq!(format!("{:?}", Op(0x8765)), "V7 <- V7 - V6");
    assert_eq!(format!("{:?}", Op(0x8876)), "V8 <- V8 >> 1 ; V7");
    assert_eq!(format!("{:?}", Op(0x8987)), "V9 <- V8 - V9");
    assert_eq!(format!("{:?}", Op(0x8678)), "invalid ; 0x8678");
    assert_eq!(format!("{:?}", Op(0x8789)), "invalid ; 0x8789");
    assert_eq!(format!("{:?}", Op(0x889A)), "invalid ; 0x889A");
    assert_eq!(format!("{:?}", Op(0x89AB)), "invalid ; 0x89AB");
    assert_eq!(format!("{:?}", Op(0x8ABC)), "invalid ; 0x8ABC");
    assert_eq!(format!("{:?}", Op(0x8BCD)), "invalid ; 0x8BCD");
    assert_eq!(format!("{:?}", Op(0x8CDE)), "VC <- VC << 1 ; VD");
    assert_eq!(format!("{:?}", Op(0x8DEF)), "invalid ; 0x8DEF");

    assert_eq!(format!("{:?}", Op(0x9210)), "skip_if V2 != V1");
    assert_eq!(format!("{:?}", Op(0x9001)), "invalid ; 0x9001");

    assert_eq!(format!("{:?}", Op(0xA123)), "i <- 0x123");
    assert_eq!(format!("{:?}", Op(0xB234)), "pc <- V0 + 0x234");
    assert_eq!(format!("{:?}", Op(0xC345)), "V3 <- rand() & 0x45");
    assert_eq!(format!("{:?}", Op(0xD45A)), "draw_sprite(x=V4, y=V5, h=10, sprite=i)");

    assert_eq!(format!("{:?}", Op(0xE123)), "invalid ; 0xE123");
    assert_eq!(format!("{:?}", Op(0xEF9E)), "skip_if key_pressed(VF)");
    assert_eq!(format!("{:?}", Op(0xEEA1)), "skip_if !key_pressed(VE)");

    assert_eq!(format!("{:?}", Op(0xFA07)), "VA <- delay_timer");
    assert_eq!(format!("{:?}", Op(0xFB0A)), "VB <- get_key()");
    assert_eq!(format!("{:?}", Op(0xF012)), "invalid ; 0xF012");
    assert_eq!(format!("{:?}", Op(0xFC15)), "delay_timer <- VC");
    assert_eq!(format!("{:?}", Op(0xFD18)), "sound_timer <- VD");
    assert_eq!(format!("{:?}", Op(0xFA1E)), "i <- i + VA");
    assert_eq!(format!("{:?}", Op(0xFB29)), "i <- sprites[VB]");
    assert_eq!(format!("{:?}", Op(0xFC33)), "i[0..3] <- bcd(VC)");
    assert_eq!(format!("{:?}", Op(0xFD55)), "i[0..=13] <- [V0..=VD]");
    assert_eq!(format!("{:?}", Op(0xFE65)), "[V0..=VE] <- i[0..=14]");
}
