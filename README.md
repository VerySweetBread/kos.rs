# Rust library for KolibriOS

Project uses [cargo-make](https://github.com/sagiegurari/cargo-make) for building steps.
Also you need a working [FASM](https://flatassembler.net/download.php).

Once installed building is trivial then: `cargo make --profile production <example name>` produces
a ready-to-use binary at root.
