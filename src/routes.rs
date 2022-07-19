use warp::{Filter, Rejection, Reply};

use crate::{
    common::{parse_str, AlertDestination, AlertResonse, ApiKeyParams, Response},
    grafana::GrafanaAlert,
};

pub fn grafana_alerts() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // POST /api/v1/grafana/alerts?apiKey=<api-key>,<api-key>
    // apiKey format: "feishu_<API_KEY>"

    // pretty_env_logger::init();

    warp::post()
        .and(warp::path!("api" / "v1" / "grafana" / "alerts"))
        .and(warp::query::<ApiKeyParams>())
        .and(warp::body::json())
        .map(|query: ApiKeyParams, _alert: GrafanaAlert| {
            let mut items = Vec::new();
            let api_keys = parse_str(&query.api_key);
            for api_key in api_keys {
                println!(
                    "type: {:?}, key: {}",
                    api_key.alert_destination, api_key.key
                );
                if api_key.alert_destination == AlertDestination::Feishu {
                    println!("{}", api_key.key);
                }

                items.push(AlertResonse {
                    status: "success".to_string(),
                    destination: api_key.alert_destination.to_string(),
                });
            }

            warp::reply::json(&(Response {
                status: "success".to_string(),
                items,
            }))
        })
}
