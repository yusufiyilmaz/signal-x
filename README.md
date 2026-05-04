# Signal-X — Port Scanner

[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![CLI](https://img.shields.io/badge/Tool-CLI-blue)](https://github.com/yusufiyilmaz/signal-x)
[![Web](https://img.shields.io/badge/Web-GUI-green)](https://github.com/yusufiyilmaz/signal-x)
[![Security](https://img.shields.io/badge/Security-Scanner-red)](https://github.com/yusufiyilmaz/signal-x)
[![Status](https://img.shields.io/badge/Status-Stable-brightgreen)](https://github.com/yusufiyilmaz/signal-x)
[![CI](https://github.com/yusufiyilmaz/signal-x/actions/workflows/ci.yml/badge.svg)](https://github.com/yusufiyilmaz/signal-x/actions)
[![Docker](https://img.shields.io/badge/Docker-Supported-2496ED?logo=docker&logoColor=white)](https://github.com/yusufiyilmaz/signal-x)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/yusufiyilmaz/signal-x)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Rust ile yazılmış, TCP port tarama, banner grabbing, servis imzası eşleştirme, OS tespiti ve güvenlik puanlama özellikleri sunan otomatik ağ güvenlik denetim aracı.

> An async network security auditor written in Rust — TCP port scanning, banner grabbing, service version detection, TTL-based OS fingerprinting, and an A–F security score. Includes a CLI, REST API (Axum), and web GUI.

---

## 📚 Akademik Bilgi / Academic Context

| | Sızma Testi Proje Ödevi |
| --- | --- |
| **Öğretim Görevlisi** | Keyvan Arasteh Abbasabad |
| **Ders Kodu & Adı** | BGT006 Sızma Testi |
| **Üniversite** | İstinye Üniversitesi |

---

## 📑 Table of Contents

- [Özellikler](#-özellikler)
- [Web GUI](#-web-gui)
- [Mimari](#-mimari)
- [Kurulum & Kullanım](#-kurulum--kullanım)
- [Testler](#-testler)
- [Port Durumları](#-port-durumları)
- [Güvenlik Puanlama](#-güvenlik-puanlama)
- [API](#-api)
- [Demo](#-demo)
- [Öğrendiklerim](#-öğrendiklerim)
- [Referanslar](#-referanslar)
- [Lisans](#-lisans)

---

## ✨ Özellikler

- **TCP port tarama** — async paralel, open/closed/filtered tespiti
- **Banner grabbing** — servis banner okuma ve versiyon tespiti
- **Servis imzası eşleştirme** — SSH, HTTP, FTP, MySQL versiyon tespiti
- **OS tespiti** — TTL analizi ile Windows/Linux/Router tahmini
- **Güvenlik puanlama** — A-F harf notu sistemi
- **CLI desteği** — `clap` ile tam komut satırı kullanımı
- **Çoklu hedef tarama** — aynı anda birden fazla IP
- **Web panel** — 5 tema, TR/EN dil, karanlık mod
- **Raporlama** — JSON, Markdown, CSV, HTML export

---

## 🖥️ Web GUI

Kullanıcı dostu web paneli ile tarama sonuçlarını görselleştirir:

- 5 değiştirilebilir tema (Cyberpunk, Matrix, Danger, Cyber, Gold)
- TR/EN dil desteği
- Karanlık/aydınlık mod
- Port detay modalı — risk bilgisi ve güvenlik önerileri
- Tarama geçmişi — `localStorage` ile kayıt
- Port grafiği, terminal modu, ağ haritası
- CSV, HTML, JSON, Markdown rapor export

---

## 🏗️ Mimari

Proje 3 ana katmandan oluşur:

- **CLI Layer** — kullanıcı etkileşimi (`clap`)
- **Scanner Engine** — TCP tarama, banner grabbing, versiyon tespiti
- **Web Layer (Axum)** — REST API + GUI entegrasyonu

```
signal-x/
├── src/
│   ├── main.rs          # CLI giriş noktası
│   ├── scanner.rs       # TCP tarama + banner + versiyon
│   ├── discovery.rs     # Ping sweep
│   ├── os_detect.rs     # TTL analizi
│   ├── report.rs        # Puanlama + rapor
│   └── web.rs           # Axum REST API
├── static/
│   └── index.html       # Web panel
├── docs/
├── .github/workflows/
├── Dockerfile
└── Cargo.toml
```

---

## 🚀 Kurulum & Kullanım

### CLI

```bash
# Tek hedef tarama
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format md
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format json

# Tüm portlar
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --all

# Özel timeout
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --timeout 500

# Ağ keşfi
cargo run -- pentest net-discover 192.168.1 --range 1-254

# Yardım
cargo run -- --help
```

### Web GUI

```bash
cargo run
```

Tarayıcıda aç: <http://127.0.0.1:3000>

### Docker

```bash
docker build -t signal-x .
docker run -p 3000:3000 signal-x
```

---

## 🧪 Testler

```bash
cargo test -p pentester
```

| Test | Açıklama |
| --- | --- |
| `test_get_service_name` | Servis adı eşleşmesi |
| `test_parse_version_ssh` | SSH versiyon parse |
| `test_parse_version_redis` | Redis banner tespiti |
| `test_parse_version_empty` | Boş banner işleme |
| `test_port_status_display` | Port durum gösterimi |
| `test_port_status_filtered_on_timeout` | Filtreli port tespiti |
| `test_scan_port_closed` | Kapalı port tarama |
| `test_banner_empty_on_closed` | Kapalı portta boş banner |
| `test_security_score_a` | Boş liste A notu almalı |
| `test_security_score_f` | Çok fazla port F notu almalı |
| `test_security_score_riskli_port` | Riskli portlar puanı düşürmeli |
| `test_generate_markdown` | Rapor doğru bilgi içermeli |

---

## 📡 Port Durumları

| Durum | Anlam |
| --- | --- |
| `open` | Bağlantı kuruldu |
| `closed` | Bağlantı reddedildi |
| `filtered` | Zaman aşımı — firewall olabilir |

---

## 🎯 Güvenlik Puanlama

Başlangıç: **100 puan**

| Kural | Puan |
| --- | --- |
| Her açık port | -5 |
| Riskli port (21, 23, 445, 3389, 6379) | -15 |

| Not | Aralık | Anlam |
| --- | --- | --- |
| **A** | 90–100 | Çok güvenli |
| **B** | 75–89  | Güvenli |
| **C** | 60–74  | Orta risk |
| **D** | 45–59  | Riskli |
| **E** | 30–44  | Tehlikeli |
| **F** | 0–29   | Çok tehlikeli |

---

## 🌐 API

| Metod | Endpoint | Açıklama |
| --- | --- | --- |
| `POST` | `/api/scan` | Tek hedef port tarama |
| `POST` | `/api/multiscan` | Çoklu hedef tarama |
| `POST` | `/api/network` | Ağ cihaz keşfi |
| `GET`  | `/api/health` | Sunucu durumu |

---

## 🎬 Demo

> Demo videosu yakında eklenecek. / Demo video coming soon.

Bu arada CLI çıktısı için örnek bir tarama:

```bash
$ cargo run -- pentest port-scan 192.168.1.1 --range 20-25 --format md

| Port | Status | Service | Banner |
|------|--------|---------|--------|
| 21   | open   | ftp     | 220 vsFTPd 3.0.3 |
| 22   | open   | ssh     | SSH-2.0-OpenSSH_8.4p1 |
| 23   | closed | -       | - |
| 25   | filtered | -     | - |

Security Score: 70/100 — Grade: C
```

---

## 🎓 Öğrendiklerim

**Async/paralel programlama** — `tokio::spawn` ile her porta ayrı görev. Yüzlerce portu aynı anda taradım.

**TCP port durumları** — open/closed/filtered tespiti sıfırdan implement ettim.

**Banner grabbing** — TCP bağlantısı açıp ilk yanıtı okuyarak servis ve versiyon bilgisi çıkardım.

**Servis imzası eşleştirme** — `SSH-2.0-OpenSSH_8.4p1` gibi bannerlardan `OpenSSH 8.4p1` parse ettim.

**TTL analizi** — Windows=128, Linux=64, Router<48. Ping çıktısını parse ederek OS tahmini yaptım.

**CLI** — `clap` ile `--range`, `--format`, `--timeout`, `--all` parametreleri implement ettim.

**Axum** — REST API, JSON işleme, çoklu endpoint ve statik dosya sunumu.

**Güvenlik** — FTP/21, Telnet/23, SMB/445, RDP/3389, Redis/6379 portlarının neden riskli olduğunu öğrendim.

---

## 📚 Referanslar

- [Nmap](https://nmap.org) — Port tarama mimarisi
- [RFC 793](https://tools.ietf.org/html/rfc793) — TCP protokolü
- [Nmap service-probes](https://svn.nmap.org/nmap/nmap-service-probes) — Servis imzası

---

## 📄 Lisans

MIT — Yusuf Yılmaz — İstinye Üniversitesi
