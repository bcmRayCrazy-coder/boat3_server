use axum::{Json, http::StatusCode};

use crate::{
    controller,
    protocol::{gpio::{GPIOValue, RemoteGPIO}, response::RemoteResponse},
};

pub async fn gpio_config(Json(body): Json<RemoteGPIO>) -> (StatusCode, Json<RemoteResponse<bool>>) {
    println!("Config pin {} mode {}", body.pin, body.mode);
    let result = controller::gpio::config(body);
    (
        StatusCode::OK,
        Json(RemoteResponse {
            ok: Some(result.is_ok()),
            error: match result {
                Ok(_) => None,
                Err(err) => Some(format!("{}", err)),
            },
            data: Some(true),
        }),
    )
}

pub async fn gpio_set(Json(body): Json<RemoteGPIO>) -> (StatusCode, Json<RemoteResponse<bool>>) {
    println!("Set pin {} to {}", body.pin, body.value.to_string());
    let result = controller::gpio::set(body.clone());

    // controller::gpio::set(body);
    (
        StatusCode::OK,
        Json(RemoteResponse {
            ok: Some(result.is_ok()),
            error: match result {
                Ok(_) => None,
                Err(err) => Some(format!("{}", err)),
            },
            data: Some(true),
        }),
    )
}

pub async fn gpio_read(
    Json(body): Json<RemoteGPIO>,
) -> (StatusCode, Json<RemoteResponse<GPIOValue>>) {
    println!("TODO: Read pin {}", body.pin);
    let result = controller::gpio::read(body);

    (
        StatusCode::OK,
        Json(RemoteResponse {
            ok: Some(result.is_ok()),
            error: match result {
                Ok(_) => None,
                Err(ref err) => Some(format!("{}", err)),
            },
            data: result.ok(),
        }),
    )
}

pub async fn gpio_reset_all() -> (StatusCode, Json<RemoteResponse<bool>>) {
    println!("Todo: Reset all gpio");
    controller::gpio::reset_all();
    (
        StatusCode::OK,
        Json(RemoteResponse {
            ok: Some(true),
            error: None,
            data: Some(true),
        }),
    )
}
