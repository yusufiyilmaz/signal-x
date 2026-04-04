use std::net::IpAddr;
use std::process::Command;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Device {
    pub ip: String,
    pub alive: bool,
}

pub fn ping(ip: IpAddr) -> bool {
    let output = Command::new("ping")
        .args(["-n", "1", "-w", "500", &ip.to_string()])
        .output();

    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

pub fn scan_network(base_ip: &str, start: u8, end: u8) -> Vec<Device> {
    let mut devices = vec![];

    for i in start..=end {
        let ip_str = format!("{}.{}", base_ip, i);
        let ip: IpAddr = match ip_str.parse() {
            Ok(ip) => ip,
            Err(_) => continue,
        };

        let alive = ping(ip);
        if alive {
            devices.push(Device {
                ip: ip_str,
                alive,
            });
        }
    }

    devices
}