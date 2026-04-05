# Signal-X

[![Rust](https://img.shields.io/badge/rust-1.70+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-12%20passed-green?style=flat-square)]()
[![Branch](https://img.shields.io/badge/branch-midterm%2Fyusuf--yilmaz-purple?style=flat-square)]()

Rust ile yazilmis otomatik ag guvenlik denetim araci. TCP port tarama, banner grabbing,
servis imzasi eslestirme, OS tespiti ve guvenlik puanlama ozellikleri sunar.

---

## Ozellikler

| Ozellik | Aciklama |
|---------|----------|
| Port Tarama | Async paralel TCP, open/closed/filtered tespiti |
| Banner Grabbing | Servis banner okuma ve versiyon tespiti |
| Servis Imzasi | SSH, HTTP, FTP, MySQL versiyon eslestirme |
| OS Tespiti | TTL analizi — Windows/Linux/Router |
| Guvenlik Puani | A-F harf notu sistemi |
| CLI | clap ile tam komut satiri desteği |
| Coklu Tarama | Birden fazla IP ayni anda |
| Web Panel | 5 tema, TR/EN, karanlık mod |
| Raporlama | JSON, Markdown, CSV, HTML |

---

## Hizli Baslangic
```bash
git clone https://github.com/yusufiyilmaz/signal-x.git
cd signal-x
cargo run
```

Web panel: **http://127.0.0.1:3000** (cargo run calistirildiktan sonra)

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

## Port Durumlari

| Durum | Anlam |
|-------|-------|
| `open` | Baglanti kuruldu — port acik |
| `closed` | Baglanti reddedildi — port kapali |
| `filtered` | Zaman asimi — firewall engelliyor olabilir |

---

## Mimari                                                                signal-x/
├── src/
│   ├── main.rs        # CLI giris noktasi (clap)
│   ├── scanner.rs     # TCP tarama + banner grabbing + versiyon tespiti
│   ├── discovery.rs   # Ping sweep ile ag cihaz kesfi
│   ├── os_detect.rs   # TTL analizi ile OS tespiti
│   ├── report.rs      # Guvenlik puanlama + Markdown rapor
│   └── web.rs         # Axum REST API + coklu hedef endpoint
├── static/
│   └── index.html     # Web panel (5 tema, TR/EN dil)
└── Cargo.toml                                                         ---

## API Endpointleri

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
cargo test
# 12 test, hepsi gecer
```

| Test | Aciklama |
|------|----------|
| `test_get_service_name` | Servis adi eslesmesi |
| `test_parse_version_ssh` | SSH versiyon parse |
| `test_parse_version_redis` | Redis banner tespiti |
| `test_parse_version_empty` | Bos banner isleme |
| `test_port_status_display` | Port durum gosterimi |
| `test_port_status_filtered_on_timeout` | Filtreli port tespiti |
| `test_scan_port_closed` | Kapali port tarama |
| `test_banner_empty_on_closed` | Kapali portta bos banner |
| `test_security_score_a` | Bos liste A notu almali |
| `test_security_score_f` | Cok fazla port F notu almali |
| `test_security_score_riskli_port` | Riskli portlar puani dusurmeli |
| `test_generate_markdown` | Rapor dogru bilgi icermeli |

---

## Ogrendiklerim

**Async/paralel programlama** — `tokio::spawn` ile her porta ayri gorev. Yuzlerce portu ayni anda taradim.

**TCP port durumlari** — Connect tarama: kurulursa `open`, reddedilirse `closed`, zaman asiminda `filtered`.

**Banner grabbing** — TCP baglantisi acip ilk yaniti okuyarak servis ve versiyon bilgisi cikardim.

**Servis imzasi eslestirme** — `SSH-2.0-OpenSSH_8.4p1` gibi bannerlardan `OpenSSH 8.4p1` parse ettim.

**TTL analizi** — Windows=128, Linux=64, Router<48. Ping ciktisini parse ederek OS tahmini yaptim.

**CLI gelistirme** — Clap ile `--range`, `--format`, `--timeout`, `--all` parametreleri implement ettim.

**Axum** — REST API, JSON isleme, coklu endpoint ve statik dosya sunumu.

**Guvenlik** — FTP/21, Telnet/23, SMB/445, RDP/3389, Redis/6379 portlarinin neden riskli oldugunu ogrendim.

---

## Referanslar

- [Nmap](https://nmap.org) — Port tarama mimarisi referansi
- [RFC 793](https://tools.ietf.org/html/rfc793) — TCP protokolu
- [Nmap service-probes](https://svn.nmap.org/nmap/nmap-service-probes) — Servis imzasi referansi

---

## Lisans

MIT
