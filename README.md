# Signal-X

[![Rust](https://img.shields.io/badge/rust-1.70+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Lisans](https://img.shields.io/badge/lisans-MIT-blue?style=flat-square)](LICENSE)
[![Testler](https://img.shields.io/badge/testler-12%20gecti-green?style=flat-square)]()

Rust ile yazılmış otomatik ağ güvenlik denetim aracı. TCP port tarama, banner grabbing, servis imzası eşleştirme, OS tespiti ve güvenlik puanlama sunar.

---

## Özellikler

| Özellik | Açıklama |
|---------|----------|
| Port Tarama | Async paralel TCP, open/closed/filtered tespiti |
| Banner Grabbing | Servis banner okuma ve versiyon tespiti |
| Servis İmzası | SSH, HTTP, FTP, MySQL versiyon eşleştirme |
| OS Tespiti | TTL analizi — Windows/Linux/Router |
| Güvenlik Puanı | A-F harf notu sistemi |
| CLI | clap ile tam komut satırı desteği |
| Çoklu Tarama | Birden fazla IP aynı anda |
| Web Panel | 5 tema, TR/EN, karanlık mod |
| Raporlama | JSON, Markdown, CSV, HTML |

---

## Hızlı Başlangıç
```bash
git clone https://github.com/yusufiyilmaz/signal-x.git
cd signal-x
cargo run
```

Web panel: **http://127.0.0.1:3000** (cargo run çalıştırıldıktan sonra)

---

## CLI Kullanım
```bash
# Port tarama - Markdown
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format md

# Port tarama - JSON
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format json

# Tüm portlar (open + closed + filtered)
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --all

# Timeout ayarla
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --timeout 500

# Ağ keşfi
cargo run -- pentest net-discover 192.168.1 --range 1-254

# Yardım
cargo run -- --help
```

---

## Port Durumları

| Durum | Anlam |
|-------|-------|
| `open` | Bağlantı kuruldu — port açık |
| `closed` | Bağlantı reddedildi — port kapalı |
| `filtered` | Zaman aşımı — firewall engelliyor olabilir |

---

## Mimari
signal-x/
├── src/
│   ├── main.rs        # CLI giriş noktası (clap)
│   ├── scanner.rs     # TCP tarama + banner grabbing + versiyon tespiti
│   ├── discovery.rs   # Ping sweep ile ağ cihaz keşfi
│   ├── os_detect.rs   # TTL analizi ile OS tespiti
│   ├── report.rs      # Güvenlik puanlama + Markdown rapor
│   └── web.rs         # Axum REST API + çoklu hedef endpoint
├── static/
│   └── index.html     # Web panel (5 tema, TR/EN dil)
└── Cargo.toml

---

## API Endpointleri

| Metod | Endpoint | Açıklama |
|-------|----------|----------|
| POST | `/api/scan` | Tek hedef port tarama |
| POST | `/api/multiscan` | Çoklu hedef tarama |
| POST | `/api/network` | Ağ cihaz keşfi |
| GET | `/api/health` | Sunucu durumu |

---

## Güvenlik Puanlama

Başlangıç: **100 puan**

| Kural | Puan |
|-------|------|
| Her açık port | -5 |
| Riskli port (21, 23, 445, 3389, 6379) | -15 |

| Not | Aralık | Anlam |
|-----|--------|-------|
| A | 90-100 | Çok güvenli |
| B | 75-89 | Güvenli |
| C | 60-74 | Orta risk |
| D | 45-59 | Riskli |
| E | 30-44 | Tehlikeli |
| F | 0-29 | Çok tehlikeli |

---

## Testler
```bash
cargo test
# 12 test, hepsi geçer
```

| Test | Açıklama |
|------|----------|
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

## Öğrendiklerim

**Async/paralel programlama** — `tokio::spawn` ile her porta ayrı görev. Yüzlerce portu aynı anda taradım.

**TCP port durumları** — Connect tarama: kurulursa `open`, reddedilirse `closed`, zaman aşımında `filtered`.

**Banner grabbing** — TCP bağlantısı açıp ilk yanıtı okuyarak servis ve versiyon bilgisi çıkardım.

**Servis imzası eşleştirme** — `SSH-2.0-OpenSSH_8.4p1` gibi banner'lardan `OpenSSH 8.4p1` parse ettim.

**TTL analizi** — Windows=128, Linux=64, Router<48. Ping çıktısını parse ederek OS tahmini yaptım.

**CLI geliştirme** — Clap ile `--range`, `--format`, `--timeout`, `--all` parametreleri implement ettim.

**Axum** — REST API, JSON işleme, çoklu endpoint ve statik dosya sunumu.

**Güvenlik** — FTP/21, Telnet/23, SMB/445, RDP/3389, Redis/6379 portlarının neden riskli olduğunu öğrendim.

---

## Referanslar

- [Nmap](https://nmap.org) — Port tarama mimarisi referansı
- [RFC 793](https://tools.ietf.org/html/rfc793) — TCP protokolü
- [Nmap service-probes](https://svn.nmap.org/nmap/nmap-service-probes) — Servis imzası referansı

---

## Lisans

MIT