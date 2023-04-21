"use strict";

export function error(start, len) {
    const view = new DataView(this.memory.buffer, start, len);
    const msg = new TextDecoder().decode(view);
    console.error(msg);
}

export function log(start, len) {
    const view = new DataView(this.memory.buffer, start, len);
    const msg = new TextDecoder().decode(view);
    console.log(msg);
}

export function panic(start, len) {
    const view = new DataView(this.memory.buffer, start, len);
    const msg = new TextDecoder().decode(view);
    console.error(msg);
    debugger;
}
