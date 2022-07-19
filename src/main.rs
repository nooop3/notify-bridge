mod routes;
mod feishu;
mod grafana;

use warp::Filter;

use crate::routes::grafana_alerts;

#[tokio::main]
async fn main() {
    let health = warp::path!("health").map(|| "OK");

    let routes = health
        .or(grafana_alerts());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
