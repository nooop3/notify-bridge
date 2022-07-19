use warp::{
    http::HeaderValue,
    hyper::{header, Body, Response},
    Filter, Rejection, Reply,
};

use crate::{
    common::{parse_str, AlertDestination, ApiKeyParams},
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
        .map(|query: ApiKeyParams, mut _alert: GrafanaAlert| {
            let api_keys = parse_str(&query.api_key);
            for api_key in api_keys {
                println!(
                    "type: {:?}, key: {}",
                    api_key.alert_destination, api_key.key
                );
                if api_key.alert_destination == AlertDestination::Feishu {
                    println!("{}", api_key.key);
                }
            }

            let mut resp = Response::new(Body::from({}));
            resp.headers_mut()
                .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
            resp
        })
}
