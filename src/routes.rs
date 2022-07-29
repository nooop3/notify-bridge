use warp::{filters::BoxedFilter, hyper::StatusCode, Filter, Rejection, Reply};

use crate::{
    common::{check_api_key, AlertDestinations, AlertKeyMap, AlertResponse, Response},
    error::FeishuFailedRequestError,
    grafana::{alert_state_to_feishu_template_color, GrafanaAlert},
    notify::feishu::post::post as feishu_post,
};

pub async fn handle_request(
    api_keys: Vec<AlertKeyMap>,
    body: GrafanaAlert,
) -> Result<impl Reply, Rejection> {
    let mut results = Vec::new();
    for api_key in api_keys {
        info!(
            "Destination: {:?}, key: {}",
            api_key.destination, api_key.key
        );
        if api_key.destination == AlertDestinations::Feishu {
            let template = alert_state_to_feishu_template_color(&body.state);
            let message = body.message.clone().unwrap_or_else(|| "".to_string());
            let message = format!(
                "**{}**\nMessage: {}\n - Metric: {}\n - Value: {}\n - State: {}",
                body.rule_name,
                message,
                body.eval_matches[0].metric,
                body.eval_matches[0].value,
                body.state,
            );
            match feishu_post(
                api_key.key,
                body.title.clone(),
                body.rule_url.clone().unwrap_or_else(|| "".to_string()),
                message,
                Some(template),
            )
            .await
            {
                Ok((status, response)) => {
                    results.push(AlertResponse {
                        destination: api_key.destination.to_string(),
                        status,
                        result: response,
                    });
                }
                Err(err) => {
                    return Err(warp::reject::custom(FeishuFailedRequestError {
                        message: err.to_string(),
                    }));
                }
            }
        }
    }

    Ok(warp::reply::json(
        &(Response {
            code: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: results,
        }),
    ))
}

// POST /api/v1/grafana/alerts?apiKey=<api-key>,<api-key>
// apiKey format: "feishu_<API_KEY>,feishu_<API_KEY>"
pub fn grafana_alerts() -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "v1" / "grafana" / "alerts"))
        .and(check_api_key())
        .and(warp::body::json())
        .and_then(handle_request)
        .boxed()
}
