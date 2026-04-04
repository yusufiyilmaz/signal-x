use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PortResult {
    pub port: u16,
    pub open: bool,
    pub service: String,
}

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

pub fn scan_port(ip: IpAddr, port: u16, timeout_ms: u64) -> PortResult {
    let addr = SocketAddr::new(ip, port);
    let timeout = Duration::from_millis(timeout_ms);
    let open = TcpStream::connect_timeout(&addr, timeout).is_ok();
    let service = if open {
        get_service_name(port)
    } else {
        String::new()
    };

    PortResult { port, open, service }
}

pub fn scan_range(ip: IpAddr, start: u16, end: u16, timeout_ms: u64) -> Vec<PortResult> {
    (start..=end)
        .map(|port| scan_port(ip, port, timeout_ms))
        .filter(|r| r.open)
        .collect()
}