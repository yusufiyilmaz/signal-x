# Kurulum

## Gereksinimler

- Rust 1.70+
- Cargo
- Windows / Linux / macOS

## Windows

`ash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/yusufiyilmaz/signal-x.git
cd signal-x
cargo run
```

## Linux / Kali

`ash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source C:\Users\yusuf/.cargo/env
git clone https://github.com/yusufiyilmaz/signal-x.git
cd signal-x
sudo cargo run
```

## Dogrulama

`ash
cargo test -p pentester
cargo clippy
```"

Set-Content docs\FAQ.md -Encoding UTF8 -Value 
