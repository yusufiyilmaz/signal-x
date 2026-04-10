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

#[derive(Parser)]
#[command(name = "signal-x")]
#[command(about = "Ag guvenlik denetim araci")]
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
        target: String,
        #[arg(long, default_value = "1-1024")]
        range: String,
        #[arg(long, default_value = "md")]
        format: String,
        #[arg(long, default_value = "200")]
        timeout: u64,
        /// Tum portlari goster (acik+kapali+filtreli)
        #[arg(long, default_value = "false")]
        all: bool,
    },
    /// Ag kesfi (ping sweep)
    NetDiscover {
        base_ip: String,
        #[arg(long, default_value = "1-254")]
        range: String,
    },
}

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
                all,
            } => {
                run_port_scan(&target, &range, &format, timeout, all).await;
            }
            PentestModule::NetDiscover { base_ip, range } => {
                run_net_discover(&base_ip, &range).await;
            }
        },
    }
}

async fn run_port_scan(target: &str, range: &str, format: &str, timeout_ms: u64, all: bool) {
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
        eprintln!("{}", "HATA: Aralik formati yanlis!".red());
        std::process::exit(1);
    }
    let start: u16 = parts[0].parse().unwrap_or(1);
    let end: u16 = parts[1].parse().unwrap_or(1024);
    println!("{}", "[*] Signal-X Port Tarama Basladi".bright_cyan());
    println!("[*] Hedef   : {}", target.bright_yellow());
    println!("[*] Aralik  : {}-{}", start, end);
    println!("[*] Timeout : {}ms", timeout_ms);
    println!("[*] Format  : {}", format.bright_yellow());
    if all {
        println!(
            "[*] Mod     : {}",
            "TUM PORTLAR (acik+kapali+filtreli)".bright_yellow()
        );
    }
    println!("{}", "─".repeat(50).bright_black());
    let ports = if all {
        scanner::scan_range_all(ip, start, end, timeout_ms).await
    } else {
        scanner::scan_range(ip, start, end, timeout_ms).await
    };
    let os_guess = os_detect::guess_os(ip).await;
    let score = report::security_score(&ports);
    match format {
        "json" => {
            let output = serde_json::json!({"target":target,"os_guess":os_guess,"security_score":score,"open_ports":ports});
            if let Ok(json) = serde_json::to_string_pretty(&output) {
                println!("{}", json);
            }
        }
        _ => {
            let md = report::generate_markdown(target, &ports, &os_guess, &score);
            println!("{}", md);
            println!("{}", "─".repeat(50).bright_black());
            for p in &ports {
                let status_str = match p.status.as_str() {
                    "open" => "[+] ACIK    ".green().to_string(),
                    "closed" => "[-] KAPALI  ".bright_black().to_string(),
                    _ => "[?] FILTRELI".yellow().to_string(),
                };
                let risk = if [21u16, 23, 445, 3389, 6379].contains(&p.port) {
                    " <<RISKLI>>".red().to_string()
                } else {
                    String::new()
                };
                let ver = if !p.version.is_empty() {
                    format!(" [{}]", p.version)
                } else {
                    String::new()
                };
                println!(
                    "    {} Port {}/tcp — {}{}{}",
                    status_str, p.port, p.service, ver, risk
                );
            }
            println!("{}", "─".repeat(50).bright_black());
            println!("[=] Guvenlik Notu: {}", score.bright_yellow());
            println!("[=] OS Tespiti  : {}", os_guess.bright_yellow());
        }
    }
}

async fn run_net_discover(base_ip: &str, range: &str) {
    use colored::Colorize;
    let parts: Vec<&str> = range.split('-').collect();
    let start: u8 = parts.first().and_then(|s| s.parse().ok()).unwrap_or(1);
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
