#![feature(box_into_inner)]
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod alert;
mod common;
mod error;
mod notify;

use warp::Filter;

use crate::{
    alert::alicloud_monitor::route::alert as alicloud_monitor_alert,
    alert::grafana::route::alert as grafana_alert, error::handle_rejection,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let health = warp::path!("health").map(|| "OK").boxed();

    let routes = health
        .or(grafana_alert())
        .or(alicloud_monitor_alert())
        .recover(handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
