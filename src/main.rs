mod scanner;
mod discovery;
mod os_detect;
mod report;
mod web;

#[tokio::main]
async fn main() {
    web::start().await;
}