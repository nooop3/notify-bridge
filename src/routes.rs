use warp::{hyper::StatusCode, Filter, Rejection, Reply};

use crate::{
    common::{check_api_key, AlertDestinations, AlertKeyMap, AlertResonse, Response},
    grafana::GrafanaAlert,
};

pub fn grafana_alerts() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // POST /api/v1/grafana/alerts?apiKey=<api-key>,<api-key>
    // apiKey format: "feishu_<API_KEY>,feishu_<API_KEY>"

    warp::post()
        .and(warp::path!("api" / "v1" / "grafana" / "alerts"))
        .and(check_api_key())
        .and(warp::body::json())
        .map(|api_keys: Vec<AlertKeyMap>, _body: GrafanaAlert| {
            let mut results = Vec::new();
            for api_key in api_keys {
                info!(
                    "Destination: {:?}, key: {}",
                    api_key.destination, api_key.key
                );
                if api_key.destination == AlertDestinations::Feishu {
                    info!("{}", api_key.key);
                }

                results.push(AlertResonse {
                    destination: api_key.destination.to_string(),
                    status: "success".to_string(),
                });
            }

            warp::reply::json(
                &(Response {
                    code: StatusCode::OK.as_u16(),
                    message: "success".to_string(),
                    data: results,
                }),
            )
        })
        .boxed()
}
