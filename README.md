# Signal-X — Otomatik Ag Guvenlik Denetcisi

Rust ile yazilmis ag guvenlik denetim araci.
TCP port tarama, cihaz kesfi, OS tespiti, guvenlik raporlama ve CLI destegi sunar.

---

## Kurulum ve Calistirma

**Gereksinimler:** Rust 1.70+, Cargo
```bash
git clone https://github.com/yusufiyilmaz/signal-x.git
cd signal-x
cargo run
```

Tarayicide ac: [http://127.0.0.1:3000](http://127.0.0.1:3000)

---

## Kullanim

### Web Panel Modu
```bash
cargo run
```

Tarayicide `http://127.0.0.1:3000` adresini ac.

### CLI Modu
```bash
# Port tarama - Markdown cikti
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format md

# Port tarama - JSON cikti
cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format json

# Ag kesfi
cargo run -- pentest net-discover 192.168.1 --range 1-254

# Yardim
cargo run -- --help
cargo run -- pentest port-scan --help
```

---

## Ozellikler

- **Port Tarama** — Async paralel TCP tarama, 200ms timeout, servis tespiti
- **Ag Kesfi** — Ping sweep ile agdaki aktif cihazlari bulma
- **OS Tespiti** — TTL analizi ile Windows / Linux / Router tahmini
- **Guvenlik Puani** — Acik portlara gore A-F harf notu
- **CLI Desteği** — Komut satirindan port tarama ve ag kesfi
- **JSON + Markdown Rapor** — Tarama sonuclarini iki formatta export etme
- **CSV Export** — Port sonuclarini CSV olarak indirme
- **HTML Rapor** — Guzel formatlı HTML rapor export
- **Web Panel** — 5 degistirilebilir tema, TR/EN dil destegi
- **Tarama Gecmisi** — Onceki taramalara geri donme
- **Port Detay Modali** — Her port icin risk bilgisi ve guvenlik onerileri

---

## Proje Yapisi
signal-x/
├── src/
│   ├── main.rs        # Giris noktasi + CLI (clap)
│   ├── scanner.rs     # Async paralel TCP port tarama + 2 unit test
│   ├── discovery.rs   # Ping sweep ile ag cihaz kesfi
│   ├── os_detect.rs   # TTL analizi ile OS tespiti
│   ├── report.rs      # Guvenlik puanlama + markdown rapor + 4 unit test
│   └── web.rs         # Axum web sunucusu, 3 API endpoint
├── static/
│   └── index.html     # 5 temali web panel
└── Cargo.toml

---

## API Endpointleri

| Metod | Endpoint | Aciklama |
|-------|----------|----------|
| POST | `/api/scan` | IP + port araligini tarar, OS + puan + rapor dondurur |
| POST | `/api/network` | IP araligindaki aktif cihazlari dondurur |
| GET | `/api/health` | Sunucu durumunu kontrol eder |

---

## Guvenlik Puanlama Sistemi

Baslangic puani: **100**

- Her acik port: **-5 puan**
- Riskli port `21, 23, 445, 3389, 6379`: **-15 puan**

| Not | Puan Araligi | Anlam |
|-----|-------------|-------|
| A | 90 – 100 | Cok guvenli |
| B | 75 – 89 | Guvenli |
| C | 60 – 74 | Orta duzey risk |
| D | 45 – 59 | Riskli |
| E | 30 – 44 | Tehlikeli |
| F | 0 – 29 | Cok tehlikeli |

---

## Testler
```bash
cargo test
```

6 unit test bulunur, hepsi basariyla gecer:

- `test_get_service_name` — servis adi eslesmesi
- `test_scan_port_closed` — kapali port tarama
- `test_security_score_a` — bos port listesi A notu almali
- `test_security_score_f` — cok fazla port F notu almali
- `test_security_score_riskli_port` — riskli portlar puani dusurmeli
- `test_generate_markdown` — rapor dogru bilgi icermeli

---

## Ogrendiklerim

**Async programlama:** Tokio ile `tokio::spawn` kullanarak her porta ayri gorev actim.
Paralel tarama sayesinde yuzlerce portu ayni anda tarayabildim.

**TCP baglanti mantigi:** Porta TCP baglantisi acmaya calismak yeterli.
Baglanti kurulursa port acik, kurulamazsa kapali. 200ms timeout ile hizli ve dogru sonuc aldim.

**TTL analizi ile OS tespiti:** Windows TTL=128, Linux TTL=64, routerlar daha dusuk deger gonderir.
Ping ciktisini parse ederek isletim sistemini tahmin ettim.

**CLI gelistirme:** Clap kutuphanesi ile arguman parse etmeyi ogrendim.
Hem web panel hem komut satirindan calisabilen esnek bir yapi kurdum.

**Axum web framework:** REST API kurmak, JSON islemek, statik dosya sunmak icin Axum kullandim.

**Rust modul sistemi:** Her sorumlulugu ayri dosyaya bolduk.

**Ownership ve borrowing:** `&str` ile `String` farkini, `Clone` ve `Copy` traitlerini kavradim.

**Guvenlik dusuncesi:** 21 (FTP), 23 (Telnet), 445 (SMB), 3389 (RDP) gibi portlarin
neden riskli sayildigini ogrendim ve puanlama sistemine dahil ettim.

---

## Lisans

MIT
