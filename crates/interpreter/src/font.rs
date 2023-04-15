#![allow(dead_code)]

const ____ : u8 = 0b0000_0000;
const ___X : u8 = 0b0001_0000;
const __X_ : u8 = 0b0010_0000;
const __XX : u8 = 0b0011_0000;
const _X__ : u8 = 0b0100_0000;
const _X_X : u8 = 0b0101_0000;
const _XX_ : u8 = 0b0110_0000;
const _XXX : u8 = 0b0111_0000;
const X___ : u8 = 0b1000_0000;
const X__X : u8 = 0b1001_0000;
const X_X_ : u8 = 0b1010_0000;
const X_XX : u8 = 0b1011_0000;
const XX__ : u8 = 0b1100_0000;
const XX_X : u8 = 0b1101_0000;
const XXX_ : u8 = 0b1110_0000;
const XXXX : u8 = 0b1111_0000;

pub const DEFAULT : &'static [[u8; 5]] = &[
    [
        _XX_,
        X__X,
        X__X,
        X__X,
        _XX_,
    ],
    [
        _X__,
        XX__,
        _X__,
        _X__,
        XXX_,
    ],
    [
        XXX_,
        ___X,
        __X_,
        _X__,
        XXXX,
    ],
    [
        XXX_,
        ___X,
        _XX_,
        ___X,
        XXX_,
    ],
    [
        X_X_,
        X_X_,
        XXXX,
        __X_,
        __X_,
    ],
    [
        XXXX,
        X___,
        XXX_,
        ___X,
        XXX_,
    ],
    [
        _XXX,
        X___,
        XXX_,
        X__X,
        _XX_,
    ],
    [
        XXXX,
        ___X,
        __X_,
        _X__,
        X___,
    ],
    [
        _XX_,
        X__X,
        _XX_,
        X__X,
        _XX_,
    ],
    [
        _XX_,
        X__X,
        _XXX,
        ___X,
        ___X,
    ],
    [
        _XX_,
        X__X,
        XXXX,
        X__X,
        X__X,
    ],
    [
        XXX_,
        X__X,
        XXX_,
        X__X,
        XXX_,
    ],
    [
        _XXX,
        X___,
        X___,
        X___,
        _XXX,
    ],
    [
        XXX_,
        X__X,
        X__X,
        X__X,
        XXXX,
    ],
    [
        XXXX,
        X___,
        XXX_,
        X___,
        XXXX,
    ],
    [
        XXXX,
        X___,
        XXX_,
        X___,
        X___,
    ],
];
