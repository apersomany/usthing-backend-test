use axum::{routing::get, Router};
use reqwest::Client;

pub mod library;

pub fn router() -> Router<Client> {
    Router::new().route("/library", get(library::get))
}
