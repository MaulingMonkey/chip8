import * as ERRNO from "./errno.mjs";

export function environ_sizes_get(o_array_len, o_buf_len) {
    const view = new DataView(this.memory.buffer);
    view.setUint32(o_array_len, 0, true);
    view.setUint32(o_buf_len, 2, true);
    return ERRNO.SUCCESS;
}

export function environ_get(environ_array, environ_buf) {
    const view = new DataView(this.memory.buffer);
    view.setUint16(environ_buf, 0); // \0\0
    return ERRNO.SUCCESS;
}

export function fd_read(fd, iovs_ptr, iovs_len) {
    return ERRNO.BADF;
}

export function fd_write(fd, iovs_ptr, iovs_len) {
    return ERRNO.BADF;
}

export function proc_exit(code) {
    throw code;
}

export function random_get(buf, len) {
    if (!(self?.crypto?.getRandomValues)) {
        return ERRNO.NOSYS;
    } else try {
        const view = new Uint8Array(this.memory.buffer, buf, len);
        self.crypto.getRandomValues(view); // https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues
        return ERRNO.SUCCESS;
    } catch {
        return ERRNO.INVAL;
    }
}
