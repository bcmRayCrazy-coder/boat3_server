use axum::{
    Router,
    routing::{get, post},
};
use boat3_server::router;

const SERVER_ADDR: &str = "0.0.0.0:10230";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(router::root::root))
        .route("/ping", get(router::ping::ping))
        .route("/gpio/config", post(router::gpio::gpio_config))
        .route("/gpio/set", post(router::gpio::gpio_set))
        .route("/gpio/read", post(router::gpio::gpio_read));

    println!("Server start at http://{}", SERVER_ADDR);
    let listener = tokio::net::TcpListener::bind(SERVER_ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
