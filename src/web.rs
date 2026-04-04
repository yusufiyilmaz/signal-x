//! # web
//! Axum tabanli web sunucusu modulu.
//! REST API endpointlerini ve statik dosya servisini yonetir.

use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use crate::discovery;
use crate::os_detect;
use crate::report;
use crate::scanner;

/// Port tarama istegi icin JSON yapisi.
#[derive(Deserialize)]
pub struct ScanRequest {
    /// Hedef IP adresi (ornek: "192.168.1.1")
    pub target: String,
    /// Tarama baslangic portu
    pub start_port: u16,
    /// Tarama bitis portu
    pub end_port: u16,
}

/// Port tarama yaniti icin JSON yapisi.
#[derive(Serialize)]
pub struct ScanResponse {
    /// Islemin basarili olup olmadigi
    pub success: bool,
    /// Hedef IP adresi
    pub target: String,
    /// Acik portlarin listesi
    pub open_ports: Vec<scanner::PortResult>,
    /// Tahmin edilen isletim sistemi
    pub os_guess: String,
    /// Guvenlik harf notu (A-F)
    pub security_score: String,
    /// Markdown formatinda guvenlik raporu
    pub report_md: String,
}

/// Ag tarama istegi icin JSON yapisi.
#[derive(Deserialize)]
pub struct NetworkScanRequest {
    /// IP adresinin ilk uc okteti (ornek: "192.168.1")
    pub base_ip: String,
    /// Son oktetin baslangic degeri
    pub start: u8,
    /// Son oktetin bitis degeri
    pub end: u8,
}

/// Ag tarama yaniti icin JSON yapisi.
#[derive(Serialize)]
pub struct NetworkScanResponse {
    /// Islemin basarili olup olmadigi
    pub success: bool,
    /// Tespit edilen aktif cihazlarin listesi
    pub devices: Vec<discovery::Device>,
}

/// POST /api/scan endpoint handler.
/// IP adresi ve port araligini alir, tarama yapip sonuclari dondurur.
async fn handle_scan(Json(body): Json<ScanRequest>) -> Json<ScanResponse> {
    let ip: IpAddr = match body.target.parse() {
        Ok(ip) => ip,
        Err(_) => {
            return Json(ScanResponse {
                success: false,
                target: body.target,
                open_ports: vec![],
                os_guess: String::new(),
                security_score: String::new(),
                report_md: "Gecersiz IP adresi".to_string(),
            });
        }
    };
    let open_ports = scanner::scan_range(ip, body.start_port, body.end_port).await;
    let os_guess = os_detect::guess_os(ip).await;
    let security_score = report::security_score(&open_ports);
    let report_md =
        report::generate_markdown(&body.target, &open_ports, &os_guess, &security_score);
    Json(ScanResponse {
        success: true,
        target: body.target,
        open_ports,
        os_guess,
        security_score,
        report_md,
    })
}

/// POST /api/network endpoint handler.
/// IP araligini alir, aktif cihazlari ping ile tespit edip dondurur.
async fn handle_network_scan(Json(body): Json<NetworkScanRequest>) -> Json<NetworkScanResponse> {
    let devices = discovery::scan_network(&body.base_ip, body.start, body.end).await;
    Json(NetworkScanResponse {
        success: true,
        devices,
    })
}

/// GET /api/health endpoint handler.
/// Sunucunun calisip calismadигını kontrol eder.
async fn handle_health() -> &'static str {
    "Signal-X calisiyor!"
}

/// Web sunucusunu baslatir, router ve endpointleri yapilandirir.
/// Varsayilan adres: http://127.0.0.1:3000
pub async fn start() {
    println!("{}", "Signal-X v1.0.0 Baslatiliyor...".bright_cyan());
    let app = Router::new()
        .route("/api/scan", post(handle_scan))
        .route("/api/network", post(handle_network_scan))
        .route("/api/health", get(handle_health))
        .fallback_service(ServeDir::new("static"));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Panel: {}", format!("http://{}", addr).bright_green());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
