use serde::Serialize;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

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

pub async fn scan_port(ip: IpAddr, port: u16) -> PortResult {
    let addr = SocketAddr::new(ip, port);
    let open = timeout(Duration::from_millis(200), TcpStream::connect(addr))
        .await
        .is_ok();

    let service = if open {
        get_service_name(port)
    } else {
        String::new()
    };

    PortResult {
        port,
        open,
        service,
    }
}

pub async fn scan_range(ip: IpAddr, start: u16, end: u16) -> Vec<PortResult> {
    let mut handles = vec![];

    for port in start..=end {
        let handle = tokio::spawn(scan_port(ip, port));
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
        use std::net::IpAddr;
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
        let result = scan_port(ip, 19999).await;
        assert_eq!(result.open, false);
    }
}
