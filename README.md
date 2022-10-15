# yios

Personal x86_64 kernel that's built with rust.

## Requirements to run
- Need to use rust nightly. Use 'rustup' for nightly installation. If encounters 'rust-lld' linker error
could roll back nightly version to '2022-10-14'. After installing nightly, to use nightly compiler in the
current Cargo directory run 'rustup override set nightly'.
- Need to use install core rust source code, 'rustup component add rust-src'.
