use axum::{
    http::HeaderValue,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::state::AppState;

mod error;
mod extractors;
mod state;
mod routes {
    pub mod message;
    pub mod stamp;
    pub mod user;
}

#[tokio::main]
async fn main() {
    // load env vars
    dotenvy::dotenv().ok();
    // build our application with a single route
    let app = Router::new()
        .route("/user/:username", get(routes::user::get_user))
        .route("/user/register", post(routes::user::register_user))
        .route("/user/login", post(routes::user::request_session))
        .route("/user/login/confirm", post(routes::user::activate_session))
        .route("/user/whoami", post(routes::user::whoami))
        .route(
            "/stamp/request_system_issue",
            post(routes::stamp::request_system_issue),
        )
        .route("/stamp/system_issue", post(routes::stamp::system_issue))
        .route(
            "/message/send_periodic",
            post(routes::message::send_periodic),
        )
        .route("/message/send_onetime", post(routes::message::send_onetime))
        .route("/message/get_all", get(routes::message::get_all_messages))
        .route("/message/:id", get(routes::message::get_message_by_id))
        .layer(Extension(AppState::new().await))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap()),
        );

    // run our app with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();
}
