"use strict";

const audio = new AudioContext();
let tone = undefined;
"keydown keypress click mousedown".split(' ').forEach(e => addEventListener(e, function(ev) { console.log(ev.type); audio.resume(); }));

export function sound_play() {
    if (tone) return; // XXX: should never happen?
    tone = audio.createOscillator();
    tone.type = "sine";
    tone.frequency.value = 440; // https://en.wikipedia.org/wiki/A440_(pitch_standard)
    tone.connect(audio.destination);
    tone.start();
}

export function sound_stop() {
    tone?.stop();
    tone = undefined;
}
