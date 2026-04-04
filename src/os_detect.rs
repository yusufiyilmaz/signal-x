//! # os_detect
//! Ping ciktisindaki TTL degerini analiz ederek hedef cihazin
//! isletim sistemini tahmin eder.

use std::net::IpAddr;
use std::process::Command;

/// Hedef IP adresine ping atar ve TTL analizine gore OS tahmin eder.
///
/// # Parametreler
/// - `ip`: Hedef IP adresi
///
/// # Donus Degeri
/// Tahmin edilen OS adi: "Windows", "Linux / macOS", "Router/Switch" veya "Tespit edilemedi"
pub async fn guess_os(ip: IpAddr) -> String {
    let output = Command::new("ping")
        .args(["-n", "1", "-w", "1000", &ip.to_string()])
        .output();
    match output {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout).to_string();
            parse_ttl(&text)
        }
        Err(_) => "Tespit edilemedi".to_string(),
    }
}

/// Ping ciktisindaki TTL degerini parse ederek OS tahmin eder.
///
/// # TTL Araliklari
/// - 100-128: Windows
/// - 49-64: Linux / macOS
/// - 1-48: Router/Switch
///
/// # Parametreler
/// - `ping_output`: Ping komutunun ham metin ciktisi
///
/// # Donus Degeri
/// Tahmin edilen isletim sistemi adi
fn parse_ttl(ping_output: &str) -> String {
    for line in ping_output.lines() {
        let lower = line.to_lowercase();
        if lower.contains("ttl=") {
            if let Some(pos) = lower.find("ttl=") {
                let ttl_str: String = lower[pos + 4..]
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect();
                if let Ok(ttl) = ttl_str.parse::<u32>() {
                    return match ttl {
                        100..=128 => "Windows".to_string(),
                        49..=64 => "Linux / macOS".to_string(),
                        1..=48 => "Router/Switch".to_string(),
                        _ => "Bilinmiyor".to_string(),
                    };
                }
            }
        }
    }
    "Tespit edilemedi".to_string()
}
