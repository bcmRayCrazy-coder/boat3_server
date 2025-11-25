use axum::{Json, http::StatusCode};

use crate::{
    controller,
    protocol::{gpio::RemoteGPIO, response::RemoteResponse},
};

pub async fn gpio_config(Json(body): Json<RemoteGPIO>) -> (StatusCode, Json<RemoteResponse<()>>) {
    println!("Todo: Config pin {} mode {}", body.pin, body.mode);
    let result = controller::gpio::config(body);
    (
        StatusCode::OK,
        Json(RemoteResponse {
            ok: Some(result.is_ok()),
            error: match result {
                Ok(_) => None,
                Err(err) => format!("{}", err),
            },
            data: None,
        }),
    )
}

pub async fn gpio_set(Json(body): Json<RemoteGPIO>) -> (StatusCode, Json<RemoteResponse<()>>) {
    println!("Todo: Set pin {} to {}", body.pin, body.value.to_string());
    let result = controller::gpio::set(body);

    controller::gpio::set(body);
    (
        StatusCode::OK,
        Json(RemoteResponse {
            ok: Some(result.is_ok()),
            error: match result {
                Ok(_) => None,
                Err(err) => format!("{}", err),
            },
            data: None,
        }),
    )
}

pub async fn gpio_read(
    Json(body): Json<RemoteGPIO>,
) -> (StatusCode, Json<RemoteResponse<RemoteGPIO>>) {
    println!("Todo: Read pin {}", body.pin);
    let result = controller::gpio::read(body);

    controller::gpio::read(body);
    (
        StatusCode::OK,
        Json(RemoteResponse {
            ok: Some(result.is_ok()),
            error: match result {
                Ok(_) => None,
                Err(err) => format!("{}", err),
            },
            data: None,
        }),
    )
}

pub async fn gpio_reset_all() -> (StatusCode, Json<RemoteResponse<()>>) {
    println!("Todo: Reset all gpio");
    controller::gpio::reset_all();
    (
        StatusCode::OK,
        Json(RemoteResponse {
            ok: Some(true),
            error: None,
            data: None,
        }),
    )
}
