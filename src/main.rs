extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod common;
mod feishu;
mod grafana;
mod routes;

use warp::Filter;

use crate::routes::grafana_alerts;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let health = warp::path!("health").map(|| "OK");

    let routes = health.or(grafana_alerts());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
