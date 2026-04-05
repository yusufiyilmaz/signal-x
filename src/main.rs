//! # Signal-X
//! Ag guvenlik denetcisi. TCP port tarama, cihaz kesfi, OS tespiti ve web paneli sunar.
//! Web panel modu: cargo run
//! CLI modu: cargo run -- pentest port-scan 192.168.1.1 --range 1-1024 --format md

mod discovery;
mod os_detect;
mod report;
mod scanner;
mod web;

use clap::{Parser, Subcommand};

/// Signal-X — Ag Guvenlik Denetcisi
#[derive(Parser)]
#[command(name = "signal-x")]
#[command(about = "Ag guvenlik denetim araci - port tarama, OS tespiti, guvenlik puanlama")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Pentest modulleri
    Pentest {
        #[command(subcommand)]
        module: PentestModule,
    },
}

#[derive(Subcommand)]
enum PentestModule {
    /// TCP port tarama + banner grabbing
    PortScan {
        /// Hedef IP adresi
        target: String,
        /// Port araligi (ornek: 1-1024)
        #[arg(long, default_value = "1-1024")]
        range: String,
        /// Cikti formati: md veya json
        #[arg(long, default_value = "md")]
        format: String,
        /// Baglanti timeout suresi (milisaniye)
        #[arg(long, default_value = "200")]
        timeout: u64,
    },
    /// Ag kesfi (ping sweep)
    NetDiscover {
        /// Ag adresi (ornek: 192.168.1)
        base_ip: String,
        /// Aralik (ornek: 1-254)
        #[arg(long, default_value = "1-254")]
        range: String,
    },
}

/// Uygulamanin giris noktasi.
/// Arguman yoksa web sunucusunu, arguman varsa CLI modunu baslatir.
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            web::start().await;
        }
        Some(Commands::Pentest { module }) => match module {
            PentestModule::PortScan {
                target,
                range,
                format,
                timeout,
            } => {
                run_port_scan(&target, &range, &format, timeout).await;
            }
            PentestModule::NetDiscover { base_ip, range } => {
                run_net_discover(&base_ip, &range).await;
            }
        },
    }
}

/// CLI port tarama modunu calistirir.
async fn run_port_scan(target: &str, range: &str, format: &str, timeout_ms: u64) {
    use colored::Colorize;

    let ip: std::net::IpAddr = match target.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("{}", "HATA: Gecersiz IP adresi!".red());
            std::process::exit(1);
        }
    };

    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        eprintln!("{}", "HATA: Aralik formati yanlis! Ornek: 1-1024".red());
        std::process::exit(1);
    }
    let start: u16 = parts[0].parse().unwrap_or(1);
    let end: u16 = parts[1].parse().unwrap_or(1024);

    println!("{}", "[*] Signal-X Port Tarama Basladi".bright_cyan());
    println!("[*] Hedef   : {}", target.bright_yellow());
    println!("[*] Aralik  : {}-{}", start, end);
    println!("[*] Timeout : {}ms", timeout_ms);
    println!("[*] Format  : {}", format.bright_yellow());
    println!("{}", "─".repeat(50).bright_black());

    let open_ports = scanner::scan_range(ip, start, end, timeout_ms).await;
    let os_guess = os_detect::guess_os(ip).await;
    let score = report::security_score(&open_ports);

    match format {
        "json" => {
            let output = serde_json::json!({
                "target": target,
                "os_guess": os_guess,
                "security_score": score,
                "open_ports": open_ports,
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        _ => {
            let md = report::generate_markdown(target, &open_ports, &os_guess, &score);
            println!("{}", md);
            println!("{}", "─".repeat(50).bright_black());
            if open_ports.is_empty() {
                println!("{}", "[+] Acik port bulunamadi.".green());
            } else {
                for p in &open_ports {
                    let risk = if [21u16, 23, 445, 3389, 6379].contains(&p.port) {
                        "[!] RISKLI".red().to_string()
                    } else {
                        "[+] ACIK".green().to_string()
                    };
                    let banner_info = if !p.banner.is_empty() {
                        format!(" | {}", p.banner.bright_black())
                    } else {
                        String::new()
                    };
                    println!(
                        "    {} Port {}/tcp — {}{}",
                        risk, p.port, p.service, banner_info
                    );
                }
            }
            println!("{}", "─".repeat(50).bright_black());
            println!("[=] Guvenlik Notu: {}", score.bright_yellow());
            println!("[=] OS Tespiti  : {}", os_guess.bright_yellow());
        }
    }
}

/// CLI ag kesfi modunu calistirir.
async fn run_net_discover(base_ip: &str, range: &str) {
    use colored::Colorize;

    let parts: Vec<&str> = range.split('-').collect();
    let start: u8 = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(1);
    let end: u8 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(254);

    println!("{}", "[*] Signal-X Ag Kesfi Basladi".bright_cyan());
    println!("[*] Ag    : {}", base_ip.bright_yellow());
    println!("[*] Aralik: {}-{}", start, end);
    println!("{}", "─".repeat(50).bright_black());

    let devices = discovery::scan_network(base_ip, start, end).await;

    if devices.is_empty() {
        println!("{}", "[-] Aktif cihaz bulunamadi.".yellow());
    } else {
        for d in &devices {
            println!("{} {}", "[+] AKTIF".green(), d.ip.bright_white());
        }
        println!("{}", "─".repeat(50).bright_black());
        println!("[=] Toplam {} aktif cihaz bulundu.", devices.len());
    }
}
