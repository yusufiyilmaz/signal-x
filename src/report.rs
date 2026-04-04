use crate::scanner::PortResult;

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

pub fn generate_markdown(
    target: &str,
    open_ports: &[PortResult],
    os_guess: &str,
    score: &str,
) -> String {
    let mut md = String::new();

    md.push_str(&format!("# Signal-X Güvenlik Raporu\n\n"));
    md.push_str(&format!("**Hedef:** {}\n", target));
    md.push_str(&format!("**İşletim Sistemi:** {}\n", os_guess));
    md.push_str(&format!("**Güvenlik Puanı:** {}\n\n", score));
    md.push_str("## Açık Portlar\n\n");
    md.push_str("| Port | Servis |\n");
    md.push_str("|------|--------|\n");

    for port in open_ports {
        md.push_str(&format!("| {} | {} |\n", port.port, port.service));
    }

    if open_ports.is_empty() {
        md.push_str("Açık port bulunamadı.\n");
    }

    md
}