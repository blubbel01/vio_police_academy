use anyhow::Result;
use std::net::Ipv4Addr;
use vio_academy_web::app::get_app;
use vio_academy_web::config::Config;

const IP_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 8088;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let app = get_app(get_config()).await?;
    let listener = tokio::net::TcpListener::bind((IP_ADDRESS, PORT)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn get_config() -> Config {
    Config {
        oauth_app_id: "wCtb_2DBobUdOgmHv3UmjvB787jowkaT7EBLQjhN".to_string(),
        oauth_app_secret: "kyU4_-vDqXDr_t7bgXJcXUzQvy2ZsDFBYmV0zKowOJWIypDDnb87yZGxCsj84VDqDNI98Sy4WmUABgFKVjcwClHuGw7gTiVFqC-MXHRqX9UihmbMqjSw3wQkkJE11VhVESyVwuC6UR3ke_uLN16cPeDQQ8pt4DbQD82foIdUutk".to_string(),
        oauth_auth_url: "https://apiv1.vio-v.com/api/oauth2/authorize".to_string(),
        oauth_token_url: "https://apiv1.vio-v.com/api/oauth2/token".to_string(),
        database_url: "mysql://vio_police_academy@127.0.0.1:3306/vio_police_academy".to_string(),
        base_url: "http://localhost:8088".to_string()
    }
}