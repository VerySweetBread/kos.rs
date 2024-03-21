# Rust library for KolibriOS

Project uses [cargo-make](https://github.com/sagiegurari/cargo-make) for building steps.
You need to install cargo-binutils: `cargo install cargo-binutils` and llvm-tools-preview: `rustup component add llvm-tools-preview` to make it work.
Also you need a working [FASM](https://flatassembler.net/download.php).

Once installed building is trivial then: `cargo make --profile production example <example name>` produces
a ready-to-use binary at root.
