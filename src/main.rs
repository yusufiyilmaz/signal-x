//! # Signal-X
//! Ag guvenlik denetcisi. TCP port tarama, cihaz kesfi, OS tespiti ve web paneli sunar.

mod discovery;
mod os_detect;
mod report;
mod scanner;
mod web;

/// Uygulamanin giris noktasi. Web sunucusunu baslatir.
#[tokio::main]
async fn main() {
    web::start().await;
}
