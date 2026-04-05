//! # scanner
//! Async paralel TCP port tarama modulu.
//! Her port icin ayri tokio gorevi olusturur, timeout ile baglanti dener.
//! Banner grabbing ve servis imzasi eslestirme yapar.

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
    /// Banner grabbing ile alinan ham banner
    pub banner: String,
    /// Banner'dan cikartilan versiyon bilgisi
    pub version: String,
}

/// Verilen port numarasina gore bilinen servis adini dondurur.
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

/// Banner metninden servis versiyonunu cikartir.
///
/// # Ornekler
/// - "SSH-2.0-OpenSSH_8.4" -> "OpenSSH 8.4"
/// - "220 ProFTPD 1.3.5" -> "ProFTPD 1.3.5"
/// - "Server: Apache/2.4.41" -> "Apache 2.4.41"
/// - "+OK Dovecot ready" -> "Dovecot"
pub fn parse_version(banner: &str, port: u16) -> String {
    if banner.is_empty() {
        return String::new();
    }
    let b = banner.trim();

    // SSH: SSH-2.0-OpenSSH_8.4p1
    if b.starts_with("SSH-") {
        let parts: Vec<&str> = b.splitn(3, '-').collect();
        if parts.len() >= 3 {
            return parts[2].replace('_', " ").trim().to_string();
        }
    }

    // FTP: 220 ProFTPD 1.3.5 Server
    if port == 21 && b.starts_with("220") {
        let rest = b.trim_start_matches("220").trim();
        let words: Vec<&str> = rest.split_whitespace().take(2).collect();
        if !words.is_empty() {
            return words.join(" ");
        }
    }

    // SMTP: 220 mail.example.com ESMTP Postfix
    if port == 25 && b.starts_with("220") {
        if b.contains("Postfix") {
            return "Postfix".to_string();
        }
        if b.contains("Sendmail") {
            return "Sendmail".to_string();
        }
        if b.contains("Exim") {
            return "Exim".to_string();
        }
    }

    // HTTP: Server: Apache/2.4.41 veya Server: nginx/1.18.0
    if b.contains("Server:") {
        if let Some(pos) = b.find("Server:") {
            let rest = b[pos + 7..].trim();
            let ver = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_end_matches(',');
            if !ver.is_empty() {
                return ver.replace('/', " ").to_string();
            }
        }
    }

    // HTTP response ilk satirinda versiyon
    if b.starts_with("HTTP/") {
        // Apache, nginx icin header'a bak
        if b.contains("Apache") {
            if let Some(pos) = b.find("Apache/") {
                let ver: String = b[pos..]
                    .split_whitespace()
                    .next()
                    .unwrap_or("Apache")
                    .replace('/', " ");
                return ver;
            }
            return "Apache".to_string();
        }
        if b.contains("nginx") {
            if let Some(pos) = b.find("nginx/") {
                let ver: String = b[pos..]
                    .split_whitespace()
                    .next()
                    .unwrap_or("nginx")
                    .replace('/', " ");
                return ver;
            }
            return "nginx".to_string();
        }
    }

    // MySQL: ilk byte'larda versiyon string
    if port == 3306 && b.len() > 5 {
        // MySQL banner'i binary, ASCII kismi cikart
        let ascii: String = b
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '.')
            .take(20)
            .collect();
        if ascii.contains('.') {
            return format!("MySQL {}", ascii.trim_matches(|c: char| !c.is_numeric()));
        }
    }

    // Redis: +PONG veya -ERR
    if port == 6379 {
        if b.starts_with("+") || b.starts_with("-") {
            return "Redis".to_string();
        }
    }

    // POP3: +OK Dovecot
    if port == 110 && b.starts_with("+OK") {
        if b.contains("Dovecot") {
            return "Dovecot".to_string();
        }
        if b.contains("Courier") {
            return "Courier".to_string();
        }
    }

    // IMAP: * OK Dovecot
    if port == 143 && b.starts_with("* OK") {
        if b.contains("Dovecot") {
            return "Dovecot".to_string();
        }
    }

    String::new()
}

/// Acik bir porttan banner bilgisi okur.
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

/// Tek bir porta async TCP baglantisi dener, aciksa banner okur ve versiyon cikartir.
pub async fn scan_port(ip: IpAddr, port: u16, timeout_ms: u64) -> PortResult {
    let addr = SocketAddr::new(ip, port);
    let result = timeout(Duration::from_millis(timeout_ms), TcpStream::connect(addr)).await;

    match result {
        Ok(Ok(stream)) => {
            let service = get_service_name(port);
            let banner = grab_banner(stream).await;
            let version = parse_version(&banner, port);
            PortResult {
                port,
                open: true,
                service,
                banner,
                version,
            }
        }
        _ => PortResult {
            port,
            open: false,
            service: String::new(),
            banner: String::new(),
            version: String::new(),
        },
    }
}

/// Belirtilen port araligini paralel olarak tarar, sadece acik portlari dondurur.
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

    #[test]
    fn test_parse_version_ssh() {
        assert_eq!(parse_version("SSH-2.0-OpenSSH_8.4p1", 22), "OpenSSH 8.4p1");
    }

    #[test]
    fn test_parse_version_empty() {
        assert_eq!(parse_version("", 80), "");
    }

    #[test]
    fn test_parse_version_redis() {
        assert_eq!(parse_version("+PONG", 6379), "Redis");
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
