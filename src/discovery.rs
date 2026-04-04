//! # discovery
//! Async paralel ping ile ag uzerindeki aktif cihazlari kesfeder.
//! Her IP icin ayri tokio gorevi olusturur.

use serde::Serialize;
use std::net::IpAddr;
use tokio::process::Command;

/// Ag uzerinde tespit edilen bir cihazi temsil eder.
#[derive(Serialize, Clone)]
pub struct Device {
    /// Cihazin IP adresi
    pub ip: String,
    /// Cihazin aktif olup olmadigi
    pub alive: bool,
}

/// Verilen IP adresine ping atar, cihazin aktif olup olmadigini dondurur.
///
/// # Parametreler
/// - `ip`: Ping atilacak IP adresi
///
/// # Donus Degeri
/// Cihaz aktifse `true`, degilse `false`
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

/// Belirtilen IP araligini paralel ping ile tarar, aktif cihazlari dondurur.
///
/// # Parametreler
/// - `base_ip`: IP adresinin ilk uc okteti (ornek: "192.168.1")
/// - `start`: Son oktetin baslangic degeri
/// - `end`: Son oktetin bitis degeri
///
/// # Donus Degeri
/// Aktif cihazlarin listesi (`Vec<Device>`)
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
