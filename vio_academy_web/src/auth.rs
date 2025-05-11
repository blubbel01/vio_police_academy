use crate::app::AppState;
use crate::error::AppError;
use axum::extract::{Query, State};
use axum::response::{Html, Redirect};
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};
use serde::Deserialize;
use tower_sessions::Session;
use tracing::debug;
use vio_v_client::apis::configuration::Configuration;
use vio_v_client::apis::default_api;

const SESSION_PKCE_KEY: &str = "oauth_pkce_verifier";
const SESSION_CSRF_KEY: &str = "oauth_csrf_token";
pub async fn login_handler(
    State(state): State<AppState>,
    session: Session,
) -> Result<Redirect, AppError> {
    let client = state.oauth_client.clone();

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let verifier_secret = pkce_verifier.secret().to_owned();

    session.insert(SESSION_PKCE_KEY, verifier_secret).await?;

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .add_scope(Scope::new("read.self".into()))
        .url();

    session
        .insert(SESSION_CSRF_KEY, csrf_token.into_secret())
        .await?;
    session.save().await?;

    Ok(Redirect::temporary(auth_url.as_ref()))
}

#[derive(Deserialize)]
pub struct AuthQuery {
    code: String,
    state: String,
}

pub async fn callback_handler(
    State(state): State<AppState>,
    Query(query): Query<AuthQuery>,
    session: Session,
) -> Result<Html<String>, AppError> {
    let verifier = session.get::<String>(SESSION_PKCE_KEY).await?;
    let csrf = session.get::<String>(SESSION_CSRF_KEY).await?;

    let mut val: Vec<String> = Vec::new();
    val.push(
        session
            .get::<String>(SESSION_PKCE_KEY)
            .await
            .unwrap()
            .unwrap_or_else(|| String::from("NONE!!")),
    );
    val.push(
        session
            .get::<String>(SESSION_CSRF_KEY)
            .await
            .unwrap()
            .unwrap_or_else(|| String::from("NONE!!")),
    );
    let a = format!("value: {:?}", val);
    println!("{}", a);

    debug!(verifier, csrf, SESSION_PKCE_KEY, SESSION_CSRF_KEY);

    if let (Some(verifier_str), Some(csrf_str)) = (verifier, csrf) {
        let pkce_verifier = PkceCodeVerifier::new(verifier_str);

        if csrf_str != query.state {
            return Err(AppError::Unauthorized);
        }

        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let token_result = state
            .oauth_client
            .exchange_code(AuthorizationCode::new(query.code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await;

        match token_result {
            Ok(token) => {
                let access_token = token.access_token().secret();
                let refresh_token = token.refresh_token().unwrap().secret();
                
                let config = Configuration {
                    bearer_access_token: Some(access_token.to_string()),
                    ..Default::default()
                };

                let user = default_api::self_get(&config).await.unwrap();
                
                debug!(user.id, "{:?}", user);

                Ok(Html(format!(
                    "Access Token: {}\n Refresh Token: {}",
                    access_token, refresh_token
                )))
            }
            Err(err) => Ok(Html(format!("Token exchange failed: {}", err))),
        }
    } else {
        Ok(Html("Missing PKCE verifier in session.".into()))
    }
}
