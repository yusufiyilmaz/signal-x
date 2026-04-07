# Signal-X Mimari Dokumani

## Genel Bakis

Signal-X, Rust ile yazilmis modüler bir ag guvenlik denetim aracidir.

## Moduller

### scanner.rs
- Async paralel TCP port tarama
- Banner grabbing
- Servis imzasi eslestirme
- open/closed/filtered port tespiti

### discovery.rs
- Ping sweep ile ag cihaz kesfi
- Paralel ping gondерimi

### os_detect.rs
- TTL analizi ile OS tespiti
- Windows/Linux/Router tahmini

### report.rs
- A-F guvenlik puanlama sistemi
- Markdown rapor uretimi

### web.rs
- Axum REST API
- Coklu hedef tarama endpoint

### main.rs
- CLI giris noktasi (clap)
- Web panel baslangici

## API Akisi
Client -> POST /api/scan -> web.rs -> scanner.rs -> report.rs -> JSON yanit
Client -> POST /api/network -> web.rs -> discovery.rs -> JSON yanit
Client -> POST /api/multiscan -> web.rs -> scanner.rs (paralel) -> JSON yanit
