//! # scanner
//! Async paralel TCP port tarama modulu.
//! Her port icin ayri tokio gorevi olusturur, timeout ile baglanti dener.
//! Banner grabbing ile servis versiyonu tespiti yapar.

use serde::Serialize;
use std::net::{IpAddr, SocketAddr};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

/// Tek bir portun tarama sonucunu temsil eder.
#[derive(Serialize, Clone)]
pub struct PortResult {
    /// Port numarasi
    pub port: u16,
    /// Portun acik olup olmadigi
    pub open: bool,
    /// Tespit edilen servis adi (HTTP, SSH, FTP vb.)
    pub service: String,
    /// Banner grabbing ile alinan versiyon bilgisi
    pub banner: String,
}

/// Verilen port numarasina gore bilinen servis adini dondurur.
///
/// # Parametreler
/// - `port`: Sorgulanacak port numarasi
///
/// # Donus Degeri
/// Bilinen servislerde servis adi, bilinmeyende "Unknown"
pub fn get_service_name(port: u16) -> String {
    match port {
        21 => "FTP".to_string(),
        22 => "SSH".to_string(),
        23 => "Telnet".to_string(),
        25 => "SMTP".to_string(),
        53 => "DNS".to_string(),
        80 => "HTTP".to_string(),
        110 => "POP3".to_string(),
        143 => "IMAP".to_string(),
        443 => "HTTPS".to_string(),
        445 => "SMB".to_string(),
        3306 => "MySQL".to_string(),
        3389 => "RDP".to_string(),
        5432 => "PostgreSQL".to_string(),
        6379 => "Redis".to_string(),
        8080 => "HTTP-Alt".to_string(),
        8443 => "HTTPS-Alt".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Acik bir porttan banner bilgisi okur.
///
/// # Parametreler
/// - `stream`: Acik TCP baglantisi
///
/// # Donus Degeri
/// Okunan banner metni, okunamazsa bos string
async fn grab_banner(mut stream: TcpStream) -> String {
    let mut buf = vec![0u8; 256];
    match timeout(Duration::from_millis(300), stream.read(&mut buf)).await {
        Ok(Ok(n)) if n > 0 => {
            let raw = &buf[..n];
            let text = String::from_utf8_lossy(raw)
                .trim()
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .filter(|c| c.is_ascii() && !c.is_ascii_control())
                .take(80)
                .collect::<String>()
                .trim()
                .to_string();
            text
        }
        _ => String::new(),
    }
}

/// Tek bir porta async TCP baglantisi dener, aciksa banner okur.
///
/// # Parametreler
/// - `ip`: Hedef IP adresi
/// - `port`: Taranacak port numarasi
/// - `timeout_ms`: Baglanti timeout suresi (milisaniye)
///
/// # Donus Degeri
/// `PortResult` yapisi (port, acik mi, servis adi, banner)
pub async fn scan_port(ip: IpAddr, port: u16, timeout_ms: u64) -> PortResult {
    let addr = SocketAddr::new(ip, port);
    let result = timeout(Duration::from_millis(timeout_ms), TcpStream::connect(addr)).await;

    match result {
        Ok(Ok(stream)) => {
            let service = get_service_name(port);
            let banner = grab_banner(stream).await;
            PortResult {
                port,
                open: true,
                service,
                banner,
            }
        }
        _ => PortResult {
            port,
            open: false,
            service: String::new(),
            banner: String::new(),
        },
    }
}

/// Belirtilen port araligini paralel olarak tarar, sadece acik portlari dondurur.
///
/// # Parametreler
/// - `ip`: Hedef IP adresi
/// - `start`: Baslangic port numarasi
/// - `end`: Bitis port numarasi
/// - `timeout_ms`: Baglanti timeout suresi (milisaniye)
///
/// # Donus Degeri
/// Acik portlarin listesi (`Vec<PortResult>`)
pub async fn scan_range(ip: IpAddr, start: u16, end: u16, timeout_ms: u64) -> Vec<PortResult> {
    let mut handles = vec![];
    for port in start..=end {
        let handle = tokio::spawn(scan_port(ip, port, timeout_ms));
        handles.push(handle);
    }
    let mut results = vec![];
    for handle in handles {
        if let Ok(result) = handle.await {
            if result.open {
                results.push(result);
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_name() {
        assert_eq!(get_service_name(80), "HTTP");
        assert_eq!(get_service_name(443), "HTTPS");
        assert_eq!(get_service_name(22), "SSH");
        assert_eq!(get_service_name(21), "FTP");
        assert_eq!(get_service_name(9999), "Unknown");
    }

    #[tokio::test]
    async fn test_scan_port_closed() {
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        let result = scan_port(ip, 19999, 200).await;
        assert_eq!(result.open, false);
    }

    #[tokio::test]
    async fn test_banner_empty_on_closed() {
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        let result = scan_port(ip, 19998, 200).await;
        assert_eq!(result.banner, "");
    }
}
