# Signal-X — Otomatik Ag Guvenlik Denetcisi

Rust ile yazilmis, web tabanli ag guvenlik denetim araci.
TCP port tarama, cihaz kesfi, OS tespiti ve guvenlik raporlama ozellikleri sunar.

---

## Kurulum ve Calistirma

**Gereksinimler:** Rust 1.70+, Cargo
```bash
# Projeyi klonla
git clone https://github.com/yusufiyilmaz/signal-x.git
cd signal-x

# Calistir
cargo run
```

Tarayicide ac: [http://127.0.0.1:3000](http://127.0.0.1:3000)

---

## Ozellikler

- **Port Tarama** — Async paralel TCP tarama, 200ms timeout, servis tespiti
- **Ag Kesfi** — Ping sweep ile agdaki aktif cihazlari bulma
- **OS Tespiti** — TTL analizi ile Windows / Linux / Router tahmini
- **Guvenlik Puani** — Acik portlara gore A-F harf notu
- **Markdown Rapor** — Tarama sonuclarini export etme
- **Web Panel** — 5 degistirilebilir tema ile tarayici arayuzu

---

## Kullanim

### Port Tarama
1. Web panelinde **Port Tarama** sekmesini ac
2. Hedef IP adresini gir → ornek: `192.168.1.1`
3. Port araligini belirle → ornek: `1` ile `1000`
4. **Tara** butonuna bas, sonuclar ekrana gelir

### Ag Kesfi
1. **Ag Kesfi** sekmesini ac
2. Ag adresini gir → ornek: `192.168.1`
3. Aralik belirle → ornek: `1` ile `254`
4. **Tara** butonuna bas, aktif cihazlar listelenir

### Rapor
1. Tarama tamamlandiktan sonra **Rapor** sekmesine gec
2. Markdown veya JSON formatinda export edebilirsin

---

## Web Panel Temaları

Sag ustteki renkli dairelerle tema degistirilebilir:

| Tema | Renk Paleti |
|------|-------------|
| Cyberpunk | Mor ve pembe neon |
| Matrix | Yesil |
| Tehlike | Kirmizi |
| Cyber | Mavi |
| Gold | Altin |

---

## Proje Yapisi
signal-x/
├── src/
│   ├── main.rs        # Giris noktasi, modulleri baglar
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
Paralel tarama sayesinde yuzlerce portu ayni anda tarayabildim; sirayla yapilsaydi cok yavas olurdu.

**TCP baglanti mantigi:** Porta TCP baglantisi acmaya calismak yeterli.
Baglanti kurulursa port acik, kurulamazsa kapali. 200ms timeout ile hizli ve dogru sonuc aldim.

**TTL analizi ile OS tespiti:** Windows TTL=128, Linux TTL=64, routerlar daha dusuk deger gonderir.
Ping ciktisini parse ederek isletim sistemini tahmin ettim.

**Axum web framework:** REST API kurmak, JSON islemek, statik dosya sunmak icin Axum kullandim.
Router yapisini ve handler fonksiyonlarini ogrendim.

**Rust modul sistemi:** Her sorumlulugu ayri dosyaya bolduk. Tek dosyada olmamasi
hem okunurlugu hem de bakimi kolaylastirdi.

**Ownership ve borrowing:** `&str` ile `String` farkini, `Clone` ve `Copy` traitlerini kavradim.
Baslangicta en zorlayici konu buydu.

**Guvenlik dusuncesi:** 21 (FTP), 23 (Telnet), 445 (SMB), 3389 (RDP) gibi portlarin
neden riskli sayildigini ogrendim ve puanlama sistemine dahil ettim.

---

## Lisans

MIT
