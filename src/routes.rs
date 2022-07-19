use serde_derive::Deserialize;

use warp::{Filter, hyper::{Body, Response, header}, http::HeaderValue, Reply, Rejection};

use crate::grafana::GrafanaAlert;

#[derive(Debug, PartialEq)]
enum AlertDestination {
    Grafana,
    Feishu,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiKeyParams {
    api_key: String,
}

struct AlertKeyMap {
    alert_destination: AlertDestination,
    key: String,
}

fn parse_str(s: &str) -> Vec<AlertKeyMap> {
    s.split(',').filter_map(|s| match s.contains('_') {
        true => {
            let mut parts = s.split('_');
            Some( AlertKeyMap {
                alert_destination: match parts.next().unwrap() {
                    "grafana" => AlertDestination::Grafana,
                    "feishu" => AlertDestination::Feishu,
                    _ => panic!("Unknown alert destination: {}", s),
                },
                key: parts.next().unwrap().to_string(),
            })
        },
        false => None,
    }).collect()
}

pub fn grafana_alerts () -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
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
                println!("type: {:?}, key: {}", api_key.alert_destination, api_key.key);
                if api_key.alert_destination == AlertDestination::Feishu {
                    println!("{}", api_key.key);
                }
            };
            let mut resp = Response::new(Body::from("Hello, world!"));
            resp.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/plain"),
            );
            resp
        })
}
