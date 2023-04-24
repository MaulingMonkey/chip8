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

async function on_load() {
    Object.assign(ctx, await wasm); // ctx.module, ctx.instance
    ctx.memory = ctx.instance.exports.memory || ctx.memory;
    ctx.instance.exports.setup();

    const scratchPageStart = ctx.memory.grow(1) * 64 * 1024; // 64 KiB (https://developer.mozilla.org/en-US/docs/WebAssembly/JavaScript_interface/Memory/grow)

    function update_context() {
        requestAnimationFrame(update_context);
        ctx.instance.exports.update();
    }
    requestAnimationFrame(update_context);

    /** @param {ArrayBuffer} arrayBuffer */
    function open_rom(arrayBuffer) {
        const rom = new Uint8Array(arrayBuffer);

        const ROM_START = 0x200;
        const ROM_END   = 0xF00;
        const ROM_MAX   = ROM_END - ROM_START;

        if (rom.length > ROM_MAX) {
            window.alert(`ROM too large to fit into CHIP-8 program memory (${rom.length} > ${ROM_MAX} bytes @ 0x200 .. 0xF00)`);
            return;
        }

        const dst = new Uint8Array(ctx.memory.buffer, scratchPageStart, ROM_MAX);
        for (let i=0; i<rom.length; ++i) dst[i] = rom[i];
        ctx.instance.exports.reset(scratchPageStart);
    }

    [...document.getElementsByTagName("input")].forEach(function (input){
        if (input.type === "file" && input.dataset.action === "open-local") {
            input.style.cursor = "pointer";
            input.addEventListener("input", async function() {
                const file = input.files?.[0];
                if (file) open_rom(await file.arrayBuffer())
            });
            return;
        }
    });

    [...document.getElementsByTagName("button")].forEach(function (button){
        const run = button.dataset.run;
        if (run) {
            button.style.cursor = "pointer";
            button.addEventListener("click", async() => open_rom(await (await fetch(run)).arrayBuffer()));
            return;
        }
    });
}

if (document.readyState === "complete") on_load();
else addEventListener("load", on_load);
