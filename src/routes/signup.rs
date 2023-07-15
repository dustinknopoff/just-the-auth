use axum::response::{Html, IntoResponse};

use crate::authentication::view::create::create;

/// Returns HTTP status code OK (200) to act as a health check
#[tracing::instrument(name = "[Signup]")]
pub async fn signup_view() -> impl IntoResponse {
    Html(create().to_string())
}
