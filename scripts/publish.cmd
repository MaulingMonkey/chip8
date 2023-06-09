@pushd "%~dp0.." && setlocal

:: ensure build up-to-date
cargo build --package maulingmonkey-chip8-website --lib --target=wasm32-wasi --release

:: prep branch/worktree
git remote add pages git@github.com:MaulingMonkey/chip8.git             2>NUL
git fetch pages gh-pages                                                || goto :die
git worktree add .worktrees\pages pages/gh-pages                        || goto :die

:: generate/populate files
cargo about generate about.hbs > .worktrees\pages\about.html            || goto :die
cd .worktrees\pages
copy /Y ..\..\crates\website\src\website.html   index.html              || goto :die
copy /Y ..\..\target\wasm32-wasi\release\maulingmonkey_chip8_website.wasm website.wasm || goto :die
robocopy ..\..\crates\website\src . *.mjs /NJH /NJS /NP
mkdir examples                                                          2>NUL
copy /Y ..\..\examples\sierpinski.ch8           examples\sierpinski.ch8 || goto :die

:: publish branch
git add -A .                                                            || goto :die
git commit -m "auto-generate gh-pages via scripts\publish.cmd"          || goto :die
git push pages HEAD:gh-pages                                            || goto :die

:: cleanup
cd ..\..                                                                || goto :die
git worktree remove .worktrees\pages                                    || goto :die
:die
@popd && endlocal && exit /B %ERRORLEVEL%
