# Signal-X

![Istinye Logo](https://upload.wikimedia.org/wikipedia/tr/6/69/%C4%B0stinye_%C3%9Cniversitesi_logo.png)

| | |
|---|---|
| **Ogrenci Adi** | Yusuf Yilmaz |
| **Ogrenci No.** | 2520191010 |
| **Danisман / Instructor** | Keyvan Arasteh Abbasabad |
| **Ders Kodu** | BGT006 Sizma Testi |

[![CI](https://github.com/yusufiyilmaz/signal-x/actions/workflows/ci.yml/badge.svg)](https://github.com/yusufiyilmaz/signal-x/actions)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Lisans](https://img.shields.io/badge/lisans-MIT-blue?style=flat-square)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Testler](https://img.shields.io/badge/testler-12%20gecti-green?style=flat-square)]()

## Table of Contents

- [Ozellikler](#ozellikler)
- [Hizli Baslangic](#hizli-baslangic)
- [CLI Kullanim](#cli-kullanim)
- [Docker](#docker)
- [Port Durumlari](#port-durumlari)
- [Mimari](#mimari)
- [API](#api)
- [Guvenlik Puanlama](#guvenlik-puanlama)
- [Testler](#testler)
- [Demo](#demo)
- [Ogrendiklerim](#ogrendiklerim)
- [Referanslar](#referanslar)
## Ozellikler

| Ozellik | Aciklama |
|---------|----------|
| Port Tarama | Async paralel TCP, open/closed/filtered tespiti |
| Banner Grabbing | Servis banner okuma ve versiyon tespiti |
| Servis Imzasi | SSH, HTTP, FTP, MySQL versiyon eslestirme |
| OS Tespiti | TTL analizi — Windows/Linux/Router |
| Guvenlik Puani | A-F harf notu sistemi |
| CLI | clap ile tam komut satiri destegi |
| Coklu Tarama | Birden fazla IP ayni anda |
| Web Panel | 5 tema, TR/EN, karanlik mod |
| Raporlama | JSON, Markdown, CSV, HTML |

---

## Hizli Baslangic

**Gereksinimler:** Rust 1.70+, Cargo

```bash
git clone https://github.com/yusufiyilmaz/signal-x.git
cd signal-x
cargo run
```

Web panel: **http://127.0.0.1:3000**

---

## CLI Kullanim

```bash
# Port tarama - Markdown
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format md

# Port tarama - JSON
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format json

# Tum portlar (open + closed + filtered)
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --all

# Timeout ayarla
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --timeout 500

# Ag kesfi
cargo run -- pentest net-discover 192.168.1 --range 1-254

# Yardim
cargo run -- --help
```

---

## Docker

```bash
docker build -t signal-x .
docker run -p 3000:3000 signal-x
```

---

## Port Durumlari

| Durum | Anlam |
|-------|-------|
| `open` | Baglanti kuruldu |
| `closed` | Baglanti reddedildi |
| `filtered` | Zaman asimi — firewall olabilir |

---

## Mimari
signal-x/
├── src/
│   ├── main.rs        # CLI giris noktasi (clap)
│   ├── scanner.rs     # TCP tarama + banner grabbing + versiyon tespiti
│   ├── discovery.rs   # Ping sweep ile ag cihaz kesfi
│   ├── os_detect.rs   # TTL analizi ile OS tespiti
│   ├── report.rs      # Guvenlik puanlama + Markdown rapor
│   └── web.rs         # Axum REST API
├── static/
│   └── index.html     # Web panel (5 tema, TR/EN dil)
├── docs/
│   └── ARCHITECTURE.md
├── .github/
│   └── workflows/
│       └── ci.yml
├── Dockerfile
└── Cargo.toml

---

## API

| Metod | Endpoint | Aciklama |
|-------|----------|----------|
| POST | `/api/scan` | Tek hedef port tarama |
| POST | `/api/multiscan` | Coklu hedef tarama |
| POST | `/api/network` | Ag cihaz kesfi |
| GET | `/api/health` | Sunucu durumu |

---

## Guvenlik Puanlama

Baslangic: **100 puan**

| Kural | Puan |
|-------|------|
| Her acik port | -5 |
| Riskli port (21, 23, 445, 3389, 6379) | -15 |

| Not | Aralik | Anlam |
|-----|--------|-------|
| A | 90-100 | Cok guvenli |
| B | 75-89 | Guvenli |
| C | 60-74 | Orta risk |
| D | 45-59 | Riskli |
| E | 30-44 | Tehlikeli |
| F | 0-29 | Cok tehlikeli |

---

## Testler

```bash
cargo test -p pentester
# 12 test, hepsi gecer
```

---

## Ogrendiklerim

**Async/paralel programlama** — tokio::spawn ile her porta ayri gorev.

**TCP port durumlari** — open/closed/filtered tespiti sifirdan implement ettim.

**Banner grabbing** — servis ve versiyon bilgisi cikardim.

**Servis imzasi** — SSH, HTTP, Redis versiyon parse ettim.

**TTL analizi** — Windows=128, Linux=64, Router<48.

**CLI** — Clap ile --range, --format, --timeout, --all parametreleri.

**Axum** — REST API, JSON isleme, coklu endpoint.

**Guvenlik** — Riskli portlarin neden tehlikeli oldugunu ogrendim.

---

## Referanslar

- [Nmap](https://nmap.org) — Port tarama mimarisi
- [RFC 793](https://tools.ietf.org/html/rfc793) — TCP protokolu
- [Nmap service-probes](https://svn.nmap.org/nmap/nmap-service-probes) — Servis imzasi

---

## Lisans

MIT — Yusuf Yilmaz — Istinye Universitesi




