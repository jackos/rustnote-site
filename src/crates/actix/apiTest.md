# Actix API Tess
## Standard Test
### tests/health_check.rs
```rust
use std::net::TcpListener;

use tokio;
#[actix_rt::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    println!("\n||||||||||||||\naddr: {}\n||||||||||||||\n", addr);

    let response = client
        .get(addr + "/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    println!("status: {}", response.status());
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind host");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind to address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
```