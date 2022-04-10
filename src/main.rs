pub mod lib;
use std::net::TcpListener;

use zero_to_prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind random port.");
    run(listener)?.await
}
