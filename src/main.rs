use std::net::Ipv4Addr;

use anyhow::Error;
use axum::{Router, routing::get};

const IP_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new().route("/", get(|| async { "Hello World!" }));

    let listener = tokio::net::TcpListener::bind((IP_ADDRESS, PORT)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
