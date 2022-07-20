use warp::{Filter, Rejection, Reply};

use crate::{
    common::{check_api_key, AlertDestination, AlertKeyMap, AlertResonse, Response},
    grafana::GrafanaAlert,
};

pub fn grafana_alerts() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // POST /api/v1/grafana/alerts?apiKey=<api-key>,<api-key>
    // apiKey format: "feishu_<API_KEY>"

    warp::post()
        .and(warp::path!("api" / "v1" / "grafana" / "alerts"))
        .and(check_api_key())
        .and(warp::body::json())
        .map(|api_keys: Vec<AlertKeyMap>, _body: GrafanaAlert| {
            let mut items = Vec::new();
            for api_key in api_keys {
                info!(
                    "Destination: {:?}, key: {}",
                    api_key.alert_destination, api_key.key
                );
                if api_key.alert_destination == AlertDestination::Feishu {
                    info!("{}", api_key.key);
                }

                items.push(AlertResonse {
                    status: "success".to_string(),
                    destination: api_key.alert_destination.to_string(),
                });
            }

            warp::reply::json(
                &(Response {
                    status: "success".to_string(),
                    items,
                }),
            )
        })
        .boxed()
}
