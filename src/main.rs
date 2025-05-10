use anyhow::Result;
use std::net::Ipv4Addr;
use vio_police_academy::app::get_app;

const IP_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 8088;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let app = get_app().await?;
    let listener = tokio::net::TcpListener::bind((IP_ADDRESS, PORT)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
