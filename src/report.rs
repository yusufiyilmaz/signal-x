//! # report
//! Guvenlik puani hesaplama ve Markdown rapor uretme modulu.
//! Acik port sayisi ve riskli portlara gore A-F arasi not verir.

use crate::scanner::PortResult;

/// Acik portlara gore guvenlik puani hesaplar ve harf notu dondurur.
pub fn security_score(open_ports: &[PortResult]) -> String {
    let riskli: Vec<u16> = vec![21, 23, 445, 3389, 6379];
    let mut puan: i32 = 100;
    puan -= (open_ports.len() as i32) * 5;
    for port in open_ports {
        if riskli.contains(&port.port) {
            puan -= 15;
        }
    }
    match puan {
        90..=100 => "A".to_string(),
        75..=89 => "B".to_string(),
        60..=74 => "C".to_string(),
        45..=59 => "D".to_string(),
        30..=44 => "E".to_string(),
        _ => "F".to_string(),
    }
}

/// Tarama sonuclarindan Markdown formatinda guvenlik raporu uretir.
pub fn generate_markdown(
    target: &str,
    open_ports: &[PortResult],
    os_guess: &str,
    score: &str,
) -> String {
    let mut md = String::new();
    md.push_str("# Signal-X Guvenlik Raporu\n\n");
    md.push_str(&format!("**Hedef:** {}\n", target));
    md.push_str(&format!("**Isletim Sistemi:** {}\n", os_guess));
    md.push_str(&format!("**Guvenlik Puani:** {}\n\n", score));
    md.push_str("## Acik Portlar\n\n");
    md.push_str("| Port | Servis | Versiyon | Banner |\n");
    md.push_str("|------|--------|----------|--------|\n");
    for port in open_ports {
        let ver = if port.version.is_empty() {
            "—".to_string()
        } else {
            port.version.clone()
        };
        let ban = if port.banner.is_empty() {
            "—".to_string()
        } else {
            port.banner.chars().take(40).collect()
        };
        md.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            port.port, port.service, ver, ban
        ));
    }
    if open_ports.is_empty() {
        md.push_str("Acik port bulunamadi.\n");
    }
    md
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::PortResult;

    fn make_port(port: u16) -> PortResult {
        PortResult {
            port,
            open: true,
            service: "Test".to_string(),
            banner: String::new(),
            version: String::new(),
        }
    }

    #[test]
    fn test_security_score_a() {
        let ports = vec![];
        assert_eq!(security_score(&ports), "A");
    }

    #[test]
    fn test_security_score_f() {
        let ports: Vec<PortResult> = (0..20).map(|i| make_port(i)).collect();
        assert_eq!(security_score(&ports), "F");
    }

    #[test]
    fn test_security_score_riskli_port() {
        let ports = vec![make_port(21), make_port(23)];
        let score = security_score(&ports);
        assert!(score == "C" || score == "D" || score == "E" || score == "F");
    }

    #[test]
    fn test_generate_markdown() {
        let ports = vec![make_port(80)];
        let md = generate_markdown("192.168.1.1", &ports, "Windows", "A");
        assert!(md.contains("192.168.1.1"));
        assert!(md.contains("Windows"));
        assert!(md.contains("80"));
    }
}
