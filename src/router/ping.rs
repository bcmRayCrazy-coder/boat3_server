use axum::{Json, http::StatusCode};

use crate::protocol::response::RemoteResponse;

pub async fn ping() -> (StatusCode, Json<RemoteResponse<()>>) {
    (
        StatusCode::OK,
        Json(RemoteResponse {
            ok: Some(true),
            error: None,
            data: None,
        }),
    )
}
