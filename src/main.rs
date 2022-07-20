extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod common;
mod feishu;
mod grafana;
mod routes;

use serde::Serialize;
use warp::{hyper::StatusCode, Filter};

use crate::routes::grafana_alerts;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let health = warp::path!("health").map(|| "OK").boxed();

    let routes = health.or(grafana_alerts()).recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    // We should have expected this... Just log and say its a 500
    eprintln!("unhandled rejection: {:?}", err);
    let code = StatusCode::INTERNAL_SERVER_ERROR;
    let message = "UNHANDLED_REJECTION";

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
