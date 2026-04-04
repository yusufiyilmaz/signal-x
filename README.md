# Signal-X — Otomatik Ag Guvenlik Denetcisi

Rust ile yazilmis, ag guvenlik denetimi yapan bir arac. Web paneli uzerinden
IP tarama, port analizi, OS tespiti ve guvenlik raporu olusturma islemleri yapilabilir.

## Ozellikler

- TCP port tarama (async paralel, tokio)
- Ag cihaz kesfı (ping sweep)
- OS tespiti (TTL analizi)
- Servis tespiti (SSH, HTTP, FTP, MySQL vb.)
- A-F guvenlik puanlama sistemi
- Markdown rapor export
- Web panel (tarayicida acilir)

## Kurulum

### Gereksinimler

- Rust 1.70+
- Cargo

### Calistirma
```bash
git clone https://github.com/yusufiyilmaz/signal-x.git
cd signal-x
cargo run
```

Tarayicida ac: http://127.0.0.1:3000

## Kullanim

### Port Tarama

Web panelinde:
1. Hedef IP adresini gir
2. Port araligini belirle
3. Tara butonuna bas

### Ag Kesfı

Web panelinde Ag Kesfı sekmesine gec:
1. Ag adresini gir (ornek: 192.168.1)
2. Baslangic ve bitis araligini belirle
3. Tara butonuna bas

### Rapor

Tarama sonrasi Rapor sekmesinden Markdown formatinda raporu kopyalayabilirsin.

## Moduller

| Modul | Aciklama |
|-------|----------|
| scanner.rs | Async paralel TCP port tarama |
| discovery.rs | Ping ile ag cihaz kesfı |
| os_detect.rs | TTL analizi ile OS tespiti |
| report.rs | Guvenlik puanlama ve rapor uretimi |
| web.rs | Axum web server ve API endpointler |

## Guvenlik Puanlama

| Puan | Anlam |
|------|-------|
| A | Cok guvenli |
| B | Guvenli |
| C | Orta |
| D | Riskli |
| E | Tehlikeli |
| F | Cok tehlikeli |

Acik port basina -5 puan, riskli port (21, 23, 445, 3389, 6379) basina -15 puan dusulur.

## Test
```bash
cargo test
```

## Ogrendiklerim

[BURASI SENIN - asagidaki sorulara kendi cevaplarini yaz]

- Bu projede ne ogrendin?
- Rust'ta en zorlayici ne oldu?
- Port tarama nasil calisiyor, simdi anlatiyor musun?
- Async programlama ne demek, ogrendin mi?

## Lisans

MIT