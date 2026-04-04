# Signal-X - Otomatik Ag Guvenlik Denetcisi

Rust ile yazilmis, ag guvenlik denetimi yapan bir arac. Web paneli uzerinden
IP tarama, port analizi, OS tespiti ve guvenlik raporu olusturma islemleri yapilabilir.

## Ozellikler

- TCP port tarama (async paralel, tokio)
- Ag cihaz kesfi (ping sweep)
- OS tespiti (TTL analizi)
- Servis tespiti (SSH, HTTP, FTP, MySQL vb.)
- A-F guvenlik puanlama sistemi
- Markdown rapor export
- Web panel (5 tema, tarayicida acilir)

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

### Ag Kesfi

Web panelinde Ag Kesfi sekmesine gec:
1. Ag adresini gir (ornek: 192.168.1)
2. Baslangic ve bitis araligini belirle
3. Tara butonuna bas

### Rapor

Tarama sonrasi Rapor sekmesinden Markdown formatinda raporu kopyalayabilirsin.

## Moduller

| Modul | Aciklama |
|-------|----------|
| scanner.rs | Async paralel TCP port tarama |
| discovery.rs | Ping ile ag cihaz kesfi |
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

Bu proje boyunca Rust dilini sifirdan ogrendim ve gercek bir guvenlik araci gelistirdim.

**Async programlama:** Tokio kutuphanesi ile async/await yapisini ogrendim. Her porta ayri
bir tokio::spawn gorevi acarak paralel tarama yapmayi basardim. Onceden portlari sirayla
tarasaydim cok yavас olurdu; paralel yapisayla yuzlerce portu ayni anda tarayabiliyorum.

**TCP baglanti mantigi:** Port taramanin temelini ogrendim. Bir porta TCP baglantisi
acmaya calismak yeterli; baglanti kurulursa port acik, kurulamazsa kapali demektir.
200ms timeout koyarak hem hizli hem de dogru sonuc aldim.

**TTL analizi ile OS tespiti:** Ping ciktisindaki TTL degeri isletim sistemine gore
degistigini kesfettim. Windows 128, Linux 64, routerlar ise daha dusuk TTL ile yanit
verir. Bu bilgiyi parse ederek OS tahmin ettim.

**Axum web framework:** Rust'ta HTTP sunucusu kurmak icin Axum kullandim. Router
tanimlamak, JSON islemek ve statik dosya sunmak icin nasil yapilandirildigini ogrendim.

**Modul sistemi:** Rust'ta kodun tek dosyada olmamasi gerektigini ogrendim. Her
sorumluluk ayri bir module bolundu: tarama, kesif, OS tespiti, raporlama ve web.

**Ownership ve borrowing:** Rust'in en zorlu konusu buydu. Bir degiskeni baska bir
fonksiyona gecerken kimin sahip oldugu meselesi baslangicta cok kafa karistirdi.
Zamanla &str ve String farkini, Clone ve Copy traitlerini kavradim.

**Guvenlik dusuncesi:** Hangi portlarin riskli oldugunu ogrendim. 21 (FTP), 23 (Telnet),
445 (SMB), 3389 (RDP) gibi portlar aciksa sistemin saldirilara acik oldugunu anliyorum.

## Lisans

MIT
