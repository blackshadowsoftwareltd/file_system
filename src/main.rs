use services::run::run;
pub mod config;
pub mod db;
pub mod ffi;
pub mod helpers;
pub mod models;
pub mod services;

#[tokio::main]
async fn main() {
    run().await;
}
