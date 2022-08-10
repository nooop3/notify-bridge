use warp::{filters::BoxedFilter, hyper::StatusCode, Filter, Rejection, Reply};

use crate::{
    alert::grafana::{message::AlertBody, transform::alert_state_to_feishu_template_color},
    common::{
        check_api_key, log_json, AlertDestinations, AlertKeyMap, NotifyResponseEnum, Response,
    },
    error::FeishuFailedRequestError,
    notify::feishu::{
        api_define::NotifyResponse as FeishuNotifyResponse, post::post as feishu_post,
    },
};

pub async fn handle_request(
    api_keys: Vec<AlertKeyMap>,
    body: AlertBody,
) -> Result<impl Reply, Rejection> {
    info!(
        "Received Grafana alert: {}",
        serde_json::to_string(&body).unwrap()
    );

    let mut results = Vec::new();
    for api_key in api_keys {
        if api_key.destination == AlertDestinations::Feishu {
            let template = alert_state_to_feishu_template_color(&body.state);
            let message = body.message.clone().unwrap_or_else(|| "".to_string());
            let eval_matches = body
                .eval_matches
                .iter()
                .map(|m| format!("- Matric: {}, Value: {}", m.metric, m.value))
                .collect::<Vec<String>>()
                .join("\n");
            let message = format!(
                "**{}**\nMessage: {}\n{}\n- State: {}",
                body.rule_name, message, eval_matches, body.state,
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
                    results.push(NotifyResponseEnum::Feishu(FeishuNotifyResponse {
                        destination: api_key.destination.to_string(),
                        status,
                        result: response,
                    }));
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
pub fn alert() -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path!("api" / "v1" / "grafana" / "alerts"))
        .and(check_api_key())
        .and(log_json())
        .and_then(handle_request)
        .boxed()
}
