# Signal-X Mimari

## Moduller

- **main.rs** - CLI giris noktasi (clap)
- **scanner.rs** - Async TCP port tarama, banner grabbing, versiyon tespiti
- **discovery.rs** - Ping sweep ile ag cihaz kesfi
- **os_detect.rs** - TTL analizi ile OS tespiti
- **report.rs** - Guvenlik puanlama (A-F) ve Markdown rapor
- **web.rs** - Axum REST API, coklu hedef endpoint

## Veri Akisi

Kullanici -> CLI/Web Panel -> web.rs -> scanner.rs -> PortResult -> report.rs -> JSON/Markdown

## API

- POST /api/scan - Tek hedef
- POST /api/multiscan - Coklu hedef
- POST /api/network - Ag kesfi
- GET /api/health - Sunucu durumu
