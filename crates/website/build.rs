use mmrbi::CommandExt;
use mmrbi::{cargo::script, env, fatal};

fn main() {
    let arch = env::req_var_lossy("CARGO_CFG_TARGET_ARCH");
    let os   = env::req_var_lossy("CARGO_CFG_TARGET_OS");

    if dbg!((arch.as_str(), os.as_str())) == ("wasm32", "wasi") {
        let src = "src/website.html";
        script::out::rerun_if_changed(src);

        let mut dst = env::req_var_path("OUT_DIR");
        dst.pop(); // out
        dst.pop(); // maulingmonkey-chip8-website-7f50999da3be207c
        dst.pop(); // build
        // dst could be:
        // • target\{debug, release}
        // • target\wasm32-wasi\{debug, release}
        dst.push("index.html");

        std::fs::copy(src, &dst).unwrap_or_else(|err| fatal!("error copying from `{}` to `{}`: {}", src, dst.display(), err));

        let src = "src";
        dst.pop();
        dst.push("src");

        mmrbi::Command::parse("cmd /K mklink /J").unwrap().arg(&dst).arg(src).status0().expect("mklink failed");
    }
}
