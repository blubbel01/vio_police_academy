use crate::error::AppError;
use crate::{auth, db};
use axum::routing::get;
use axum::{Extension, Router};
use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{
    AuthType, AuthUrl, Client, ClientId, ClientSecret, EmptyExtraTokenFields, EndpointNotSet,
    EndpointSet, RedirectUrl, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
};
use std::sync::Arc;
use tower_sessions::cookie::SameSite;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use crate::config::Config;

type OAuthClientWithAuthAndToken = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Clone)]
pub struct AppState {
    pub oauth_client: Arc<OAuthClientWithAuthAndToken>,
}

pub async fn get_app(config: Config) -> Result<Router, AppError> {
    let db = db::get_db_connection(&config.database_url).await?;

    let client_id = ClientId::new(config.oauth_app_id);
    let client_secret = ClientSecret::new(config.oauth_app_secret);
    let redirect_uri = RedirectUrl::new(format!("{}/auth/callback", config.base_url))?;
    let auth_url = AuthUrl::new(config.oauth_auth_url)?;
    let token_url = TokenUrl::new(config.oauth_token_url)?;

    let oauth_client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_redirect_uri(redirect_uri)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_auth_type(AuthType::BasicAuth);

    let session_store = MemoryStore::default(); // In-memory for now
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_path("/")
        .with_same_site(SameSite::None)
        .with_always_save(true);

    let app_state = AppState {
        oauth_client: oauth_client.into(),
    };

    let router = Router::new()
        .route("/", get(crate::templates::index))
        .route("/auth/login", get(auth::login_handler))
        .route("/auth/callback", get(auth::callback_handler))
        .layer(axum_htmx::auto_vary::AutoVaryLayer)
        .layer(session_layer)
        .layer(Extension(db))
        .with_state(app_state);
    Ok(router)
}