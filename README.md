# Signal-X — Otomatik Ag Guvenlik Denetcisi

Rust ile yazilmis ag guvenlik denetim araci. TCP port tarama, banner grabbing,
servis imzasi eslestirme, OS tespiti, guvenlik puanlama ve web paneli sunar.

---

## Hizli Baslangic

**Gereksinimler:** Rust 1.70+, Cargo

Tarayicide ac: http://127.0.0.1:3000 (cargo run calistirildiktan sonra)

---

## Kullanim

### Web Panel
cargo run

### CLI

Port tarama Markdown:
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format md

Port tarama JSON:
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format json

Tum portlar (acik+kapali+filtreli):
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --all

Timeout ayarla:
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --timeout 500

Ag kesfi:
cargo run -- pentest net-discover 192.168.1 --range 1-254

---

## Ozellikler

- Port Tarama: Async paralel TCP tarama, open/closed/filtered port tespiti
- Banner Grabbing: Porta baglanip servis banner bilgisini okuma
- Servis Imzasi: Banner dan versiyon tespiti (OpenSSH 8.4, Apache 2.4 vb.)
- Ag Kesfi: Ping sweep ile agdaki aktif cihazlari bulma
- OS Tespiti: TTL analizi ile Windows/Linux/Router tahmini
- Guvenlik Puani: Acik portlara gore A-F harf notu
- CLI Destegi: clap ile komut satirindan tam kullanim
- Coklu Tarama: Birden fazla IP adresini ayni anda tarama
- Raporlama: JSON, Markdown, CSV, HTML formatlarinda export
- Web Panel: 5 tema, TR/EN dil, karanlik mod, tarama gecmisi

---

## Port Durumlari

- open: Baglanti kuruldu, port acik
- closed: Baglanti reddedildi, port kapali
- filtered: Zaman asimi, firewall engelliyor olabilir

---

## Proje Yapisi

src/main.rs        - Giris noktasi + CLI (clap)
src/scanner.rs     - Async TCP port tarama + banner grabbing + versiyon tespiti
src/discovery.rs   - Ping sweep ile ag cihaz kesfi
src/os_detect.rs   - TTL analizi ile OS tespiti
src/report.rs      - Guvenlik puanlama + Markdown rapor
src/web.rs         - Axum REST API + coklu hedef tarama
static/index.html  - Web panel (5 tema, TR/EN)
Cargo.toml

---

## API Endpointleri

POST /api/scan       - Tek hedef port tarama
POST /api/multiscan  - Coklu hedef port tarama
POST /api/network    - Ag cihaz kesfi
GET  /api/health     - Sunucu durumu

---

## Guvenlik Puanlama

Baslangic puani: 100
Her acik port: -5 puan
Riskli port (21,23,445,3389,6379): -15 puan

A: 90-100 Cok guvenli
B: 75-89  Guvenli
C: 60-74  Orta risk
D: 45-59  Riskli
E: 30-44  Tehlikeli
F: 0-29   Cok tehlikeli

---

## Testler

cargo test

12 unit test, hepsi basariyla gecer:
- test_get_service_name
- test_parse_version_ssh
- test_parse_version_redis
- test_parse_version_empty
- test_port_status_display
- test_port_status_filtered_on_timeout
- test_scan_port_closed
- test_banner_empty_on_closed
- test_security_score_a
- test_security_score_f
- test_security_score_riskli_port
- test_generate_markdown

---

## Ogrendiklerim

Async programlama: Tokio ile tokio::spawn kullanarak her porta ayri gorev actim.
Paralel tarama sayesinde yuzlerce portu ayni anda tarayabildim.

TCP baglanti mantigi ve port durumlari: Baglanti kurulursa open, reddedilirse closed,
zaman asimi olursa filtered (firewall). Bu uclu sistemi sifirdan implement ettim.

Banner grabbing: Porta TCP baglantisi acip ilk yaniti okuyarak servis bilgisi aldim.
SSH, HTTP, FTP, MySQL gibi servislerin banner formatlarini ogrendim.

Servis imzasi eslestirme: Banner metninden versiyon numarasi cikardim.
SSH-2.0-OpenSSH_8.4p1 gibi bannerlardan OpenSSH 8.4p1 versiyonunu tespit ettim.

TTL analizi ile OS tespiti: Windows TTL=128, Linux TTL=64, routerlar daha dusuk deger gonderir.

CLI gelistirme: Clap kutuphanesi ile --range, --format, --timeout, --all parametrelerini implement ettim.

Axum web framework: REST API, JSON isleme, coklu hedef endpoint ve statik dosya sunumu.

Rust modul sistemi ve ownership: Her sorumlulugu ayri module bolduk.

Guvenlik dusuncesi: Riskli portlarin (21/FTP, 23/Telnet, 445/SMB, 3389/RDP, 6379/Redis)
neden tehlikeli oldugunu ogrendim ve puanlama sistemine dahil ettim.

---

## Referanslar

- Nmap kaynak kodu - port tarama mimarisi icin
- RFC 793 (TCP) - TCP baglanti protokolu
- Nmap service-probes - servis imzasi eslestirme icin

---

## Lisans

MIT

