"use strict";

const w = 64, h = 32;
const canvas = document.getElementsByTagName("canvas")[0];
const framebuffer = new ImageData(w, h);

export function render(screen_ptr) {
    const dst = new DataView(framebuffer.data.buffer);
    const src = new DataView(this.memory.buffer, screen_ptr, w*h/8);
    for (let y=0; y<h; ++y) {
        const row = src.getBigUint64(8*y);
        for (let x=0; x<w; ++x) {
            const rgba = (row & (1n << BigInt(x))) ? 0xFFFFFFFF : 0x000000FF;
            dst.setUint32(4*(w*y+x), rgba);
        }
    }
    canvas.getContext("2d").putImageData(framebuffer, 0, 0);
}
