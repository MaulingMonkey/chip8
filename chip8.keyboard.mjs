"use strict";

let codes = {};
addEventListener("keydown", function(ev) { codes[ev.code] = true; });
addEventListener("keyup",   function(ev) { delete codes[ev.code]; });
addEventListener("blur",    function(ev) { codes = {}; });

export function get_key() {
    for (let i=0; i<16; ++i) {
        const key = "X123QWEASDZC4RFV"[i];
        if (codes[key] || codes[`Key${key}`]) return i;
    }
    return 0xFFFFFFFF; // no key held
}

/** @param {number} key */
export function is_pressed(i) {
    // Typical CHIP-8 keyboard layout: https://www.google.com/search?q=chip8+keyboard+layout&tbm=isch
    // 1 2 3 C
    // 4 5 6 D
    // 7 8 9 E
    // A 0 B F
    const key = "X123QWEASDZC4RFV"[i];
    return (codes[key] || codes[`Key${key}`]) ? 1 : 0;
}
