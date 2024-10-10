use anyhow::Error;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

pub mod schedule;

pub struct AnyhowError {
    error: Error,
}

impl IntoResponse for AnyhowError {
    fn into_response(self) -> Response {
        let response = (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.error),
        );
        response.into_response()
    }
}
impl<E> From<E> for AnyhowError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self {
            error: error.into(),
        }
    }
}
