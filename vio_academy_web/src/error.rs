use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("Template rendering failed: {0}")]
    TemplateRenderError(#[from] askama::Error),

    #[error("OAuth2 parse error: {0}")]
    OAuthParseError(#[from] oauth2::url::ParseError),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Session error: {0}")]
    Session(#[from] tower_sessions::session::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{:?}", self);

        let status = match &self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EnvVar(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TemplateRenderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::OAuthParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Session(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Reqwest(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = format!("Internal Server Error: {}", self);

        (status, body).into_response()
    }
}
