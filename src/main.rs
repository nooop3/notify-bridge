#![feature(box_into_inner)]
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod alerts;
mod common;
mod error;
mod feishu;
mod grafana;
mod routes;

use warp::Filter;

use crate::{error::handle_rejection, routes::grafana_alerts};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let health = warp::path!("health").map(|| "OK").boxed();

    let routes = health.or(grafana_alerts()).recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
