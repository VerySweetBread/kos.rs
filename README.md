# Hello-world example for KolibriOS

Project uses [cargo-make](https://github.com/sagiegurari/cargo-make) for building steps.
Also you need a working [FASM](https://flatassembler.net/download.php).

Once installed building is trivial then: `cargo objcopy --release --example hwa -- -O binary --binary-architecture=i386:x86 rust.kex` produces
a ready-to-use binary at root.
