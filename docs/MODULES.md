# Modul Dokumantasyonu

## scanner.rs

Async paralel TCP port tarama modulü.

### Fonksiyonlar

- scan_port(ip, port, timeout_ms) - Tek port tarama
- scan_range(ip, start, end, timeout_ms) - Port araligi tarama
- scan_range_all(ip, start, end, timeout_ms) - Tum durumlar
- grab_banner(stream) - Banner okuma
- parse_version(banner, port) - Versiyon tespiti
- get_service_name(port) - Servis adi

## discovery.rs

Ping sweep ile ag cihaz kesfi.

## os_detect.rs

TTL analizi ile OS tespiti.
- Windows: TTL=128
- Linux: TTL=64
- Router: TTL<48

## report.rs

Guvenlik puanlama ve Markdown rapor.

## web.rs

Axum REST API.
- POST /api/scan
- POST /api/multiscan
- POST /api/network
- GET /api/health
