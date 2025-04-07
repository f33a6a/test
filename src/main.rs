use std::{env, process};
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let url = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: connect_check <URL>");
        process::exit(1);
    });
    reqwest::Client::new()
        .head(&url)
        .send()
        .await
        .map(|resp| println!("✅ Connected (Status: {})", resp.status()))
        .unwrap_or_else(|e| eprintln!("❌ Failed: {}", e));
}
