# Kullanim Kilavuzu

## Web Panel

cargo run
Tarayici: http://127.0.0.1:3000

## CLI

### Port Tarama
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format md
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format json
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --all
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --timeout 500

### Ag Kesfi
cargo run -- pentest net-discover 192.168.1 --range 1-254

## Port Durumlari

- open: Baglanti kuruldu
- closed: Baglanti reddedildi
- filtered: Zaman asimi, firewall olabilir
