use std::net::IpAddr;
use tokio::process::Command;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Device {
    pub ip: String,
    pub alive: bool,
}

pub async fn ping(ip: IpAddr) -> bool {
    let output = Command::new("ping")
        .args(["-n", "1", "-w", "500", &ip.to_string()])
        .output()
        .await;

    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

pub async fn scan_network(base_ip: &str, start: u8, end: u8) -> Vec<Device> {
    let mut handles = vec![];

    for i in start..=end {
        let ip_str = format!("{}.{}", base_ip, i);
        let handle = tokio::spawn(async move {
            let ip: IpAddr = match ip_str.parse() {
                Ok(ip) => ip,
                Err(_) => return None,
            };
            let alive = ping(ip).await;
            if alive {
                Some(Device { ip: ip_str, alive })
            } else {
                None
            }
        });
        handles.push(handle);
    }

    let mut devices = vec![];
    for handle in handles {
        if let Ok(Some(device)) = handle.await {
            devices.push(device);
        }
    }

    devices
}