use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use colored::Colorize;

use crate::scanner;
use crate::discovery;
use crate::os_detect;
use crate::report;

#[derive(Deserialize)]
pub struct ScanRequest {
    pub target: String,
    pub start_port: u16,
    pub end_port: u16,
}

#[derive(Serialize)]
pub struct ScanResponse {
    pub success: bool,
    pub target: String,
    pub open_ports: Vec<scanner::PortResult>,
    pub os_guess: String,
    pub security_score: String,
    pub report_md: String,
}

#[derive(Deserialize)]
pub struct NetworkScanRequest {
    pub base_ip: String,
    pub start: u8,
    pub end: u8,
}

#[derive(Serialize)]
pub struct NetworkScanResponse {
    pub success: bool,
    pub devices: Vec<discovery::Device>,
}

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
    let report_md = report::generate_markdown(&body.target, &open_ports, &os_guess, &security_score);

    Json(ScanResponse {
        success: true,
        target: body.target,
        open_ports,
        os_guess,
        security_score,
        report_md,
    })
}

async fn handle_network_scan(Json(body): Json<NetworkScanRequest>) -> Json<NetworkScanResponse> {
    let devices = discovery::scan_network(&body.base_ip, body.start, body.end).await;
    Json(NetworkScanResponse {
        success: true,
        devices,
    })
}

async fn handle_health() -> &'static str {
    "Signal-X calisiyor!"
}

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