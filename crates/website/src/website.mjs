"use strict";
import chip8 from "./chip8.mjs";
import * as console from "./console.mjs";
import * as wasi_snapshot_preview1 from "./wasi_snapshot_preview1.mjs";



const ctx = { memory: new WebAssembly.Memory({ initial: 2 }) };
const imports = {
    chip8,
    console,
    env: { memory: ctx.memory },
    wasi_snapshot_preview1,
};
self.imports = imports; // XXX

Object.keys(imports).forEach(m => imports[m] = {...imports[m]}); // Module -> Object
Object.values(imports).forEach(m => Object.keys(m).forEach(fn => { // function -> ctx-bound function
    if ("bind" in m[fn]) m[fn] = m[fn].bind(ctx);
}));

let wasmUrl = "website.wasm";
switch (new URLSearchParams(location.search).get("target")) {
    case "debug":   wasmUrl = "../../../target/wasm32-wasi/debug/maulingmonkey_chip8_website.wasm"; break;
    case "release": wasmUrl = "../../../target/wasm32-wasi/release/maulingmonkey_chip8_website.wasm"; break;
}
const wasm = WebAssembly.instantiateStreaming(fetch(wasmUrl), imports);

async function on_chip8_load() {
    const canvas = document.getElementsByTagName("canvas")[0];
    const framebuffer = new ImageData(64, 32);
    Object.assign(ctx, await wasm); // ctx.module, ctx.instance
    ctx.memory = ctx.instance.exports.memory || ctx.memory;

    ctx.instance.exports.setup();

    function animation_frame() {
        const dst = new DataView(framebuffer.data.buffer);
        try {
            const src_start = ctx.instance.exports.lock_memory_range(0x0F00, 0x1000);
            const src = new DataView(ctx.memory.buffer, src_start, 0x0100);
            for (let y=0; y<32; ++y) {
                const row = src.getBigUint64(8*y);
                for (let x=0; x<64; ++x) {
                    const rgba = (row & (1n << BigInt(x))) ? 0xFFFFFFFF : 0x000000FF;
                    dst.setUint32(4*(64*y+x), rgba);
                }
            }
        } finally {
            ctx.instance.exports.unlock_memory_range();
        }

        canvas.getContext("2d").putImageData(framebuffer, 0, 0);
        requestAnimationFrame(animation_frame);
    }
    animation_frame();

    setInterval(function update_context() {
        ctx.instance.exports.context_try_step_many(500/60); // 500 Hz target
        ctx.instance.exports.context_step_clocks(1);
    }, 1000/60);
}

if (document.readyState === "complete") on_chip8_load();
else addEventListener("load", on_chip8_load);
