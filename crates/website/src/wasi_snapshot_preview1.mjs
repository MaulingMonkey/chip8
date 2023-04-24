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

export function fd_read(fd, iovs_ptr, iovs_len, o_read) {
    return ERRNO.BADF;
}

export function fd_write(fd, ciovs_ptr, ciovs_len, o_write) {
    return this.instance.exports["wasi_snapshot_preview1.fd_write"](fd, ciovs_ptr, ciovs_len, o_write);
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

const CLOCKID = {
    // Wall time / time since unix epoch
    REALTIME: 0,
    // Performance time
    MONOTONIC: 1,
    // Process time
    PROCESS_CPUTIME_ID: 2,
    // Thread time
    THREAD_CPUTIME_ID: 3,
};

export function clock_time_get(clock_id, precision, o_timestamp) {
    var now     = 0;
    var errno   = ERRNO.SUCCESS;
    switch (clock_id) {
        // https://developer.mozilla.org/en-US/docs/Web/API/Performance/now#performance.now_vs._date.now
        case CLOCKID.REALTIME:  now = Date.now();           break;
        case CLOCKID.MONOTONIC: now = performance.now();    break;
        default:                errno = ERRNO.INVAL;        break;
    }
    now = BigInt(Math.round(now * 1000000)); // milliseconds → nanoseconds
    new BigUint64Array(this.memory.buffer, o_timestamp, 1)[0] = now;
    return errno;
}
