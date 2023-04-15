"use strict";
let raf_handle = undefined;
let memory = new WebAssembly.Memory({ initial: 2 });
const wasm = WebAssembly.instantiateStreaming(fetch("maulingmonkey_chip8_website.wasm"), {
    console: {
        error: function console_error(start, len) {
            const view = new DataView(memory.buffer, start, len);
            const msg = new TextDecoder().decode(view);
            console.error(msg);
        },
        log: function console_log(start, len) {
            const view = new DataView(memory.buffer, start, len);
            const msg = new TextDecoder().decode(view);
            console.log(msg);
        },
        panic: function console_panic(start, len) {
            const view = new DataView(memory.buffer, start, len);
            const msg = new TextDecoder().decode(view);
            console.error(msg);
            debugger;
        },
    },
    env: { memory },
    wasi_snapshot_preview1: {
        environ_sizes_get: function environ_sizes_get(o_array_len, o_buf_len) {
            const view = new DataView(memory.buffer);
            view.setUint32(o_array_len, 0, true);
            view.setUint32(o_buf_len, 2, true);
            return 0; // ERRNO_SUCCESS
        },
        environ_get: function environ_get(environ_array, environ_buf) {
            const view = new DataView(memory.buffer);
            view.setUint16(environ_buf, 0); // \0\0
            return 0; // ERRNO_SUCCESS
        },
        fd_read: function fd_read(fd, iovs_ptr, iovs_len) {
            return 8; // ERRNO_BADF
        },
        fd_write: function fd_write(fd, iovs_ptr, iovs_len) {
            return 8; // ERRNO_BADF
        },
        proc_exit: function proc_exit(code) {
            cancelAnimationFrame(raf_handle);
            throw code;
        },
    },
    // TODO: other imports?
});

addEventListener("load", async function on_chip8_load() {
    const canvas = document.getElementsByTagName("canvas")[0];
    const { module, instance } = await wasm;
    memory = instance.exports.memory || memory;

    instance.exports.setup();

    function animation_frame() {
        raf_handle = requestAnimationFrame(animation_frame);

        const id    = new ImageData(64, 32);
        const dst   = new DataView(id.data.buffer);

        try {
            const src   = new DataView(memory.buffer, instance.exports.lock_memory_range(0x0F00, 0x1000), 0x0100);
            for (let y=0; y<32; ++y) {
                const row = src.getBigUint64(8*y);
                for (let x=0; x<64; ++x) {
                    const rgba = (row & (1n << BigInt(x))) ? 0xFFFFFFFF : 0x000000FF;
                    dst.setUint32(4*(64*y+x), rgba);
                }
            }
        } finally {
            instance.exports.unlock_memory_range();
        }

        canvas.getContext("2d").putImageData(id, 0, 0);
    }
    animation_frame();

    setInterval(function update_context() {
        instance.exports.context_try_step_many(500/60); // 500 Hz target
        instance.exports.context_step_clocks(1);
    }, 1000/60);
});
