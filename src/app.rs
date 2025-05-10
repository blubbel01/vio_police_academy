use crate::error::AppError;
use crate::{auth, db};
use askama::Template;
use axum::response::Html;
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

pub async fn get_app() -> Result<Router, AppError> {
    let db = db::get_db_connection().await?;

    let client_id = ClientId::new("wCtb_2DBobUdOgmHv3UmjvB787jowkaT7EBLQjhN".into());
    let client_secret = ClientSecret::new("kyU4_-vDqXDr_t7bgXJcXUzQvy2ZsDFBYmV0zKowOJWIypDDnb87yZGxCsj84VDqDNI98Sy4WmUABgFKVjcwClHuGw7gTiVFqC-MXHRqX9UihmbMqjSw3wQkkJE11VhVESyVwuC6UR3ke_uLN16cPeDQQ8pt4DbQD82foIdUutk".into());
    let redirect_uri = RedirectUrl::new("http://localhost:8088/auth/callback".into())?;
    let auth_url = AuthUrl::new("https://apiv1.vio-v.com/api/oauth2/authorize".into())?;
    let token_url = TokenUrl::new("https://apiv1.vio-v.com/api/oauth2/token".into())?;

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
        .route("/", get(index))
        .route("/auth/login", get(auth::login_handler))
        .route("/auth/callback", get(auth::callback_handler))
        .layer(session_layer)
        .layer(Extension(db))
        .with_state(app_state);
    Ok(router)
}

#[derive(Template)]
#[template(path = "index.html")] // Specify the path to the index.html template file
pub struct IndexTemplate {}

async fn index() -> Html<String> {
    Html(IndexTemplate {}.render().unwrap())
}
