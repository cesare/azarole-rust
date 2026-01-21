use actix_session::{SessionGetError, SessionInsertError};
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use sqlx::error::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("record not found")]
    RecordNotFound,

    #[error("query failed")]
    QueryFailed,
}

impl From<SqlxError> for DatabaseError {
    fn from(value: SqlxError) -> Self {
        match value {
            SqlxError::RowNotFound => Self::RecordNotFound,
            _ => Self::QueryFailed,
        }
    }
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum PerRequestError {
    #[error("not found")]
    NotFound,

    #[error("unauthorized")]
    Unauthorized,

    #[error("server error")]
    ServerError,
}

impl ResponseError for PerRequestError {
    fn error_response(&self) -> HttpResponse {
        let response_json = json!({
            "error": self.to_string(),
        });

        HttpResponse::build(self.status_code()).json(response_json)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DatabaseError> for PerRequestError {
    fn from(value: DatabaseError) -> Self {
        match value {
            DatabaseError::RecordNotFound => Self::NotFound,
            _ => Self::ServerError,
        }
    }
}

impl From<SessionGetError> for PerRequestError {
    fn from(_value: SessionGetError) -> Self {
        Self::ServerError
    }
}

impl From<SessionInsertError> for PerRequestError {
    fn from(_value: SessionInsertError) -> Self {
        Self::ServerError
    }
}

impl From<anyhow::Error> for PerRequestError {
    fn from(_value: anyhow::Error) -> Self {
        Self::ServerError
    }
}
