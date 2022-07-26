use reqwest::StatusCode;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

use crate::{
    alert::alicloud_monitor::{
        message::AlertBody,
        transform::{
            event_level_to_feishu_template_color, threshold_alert_state_to_feishu_template_color,
        },
    },
    common::{
        check_api_key, log_form, AlertDestinations, AlertKeyMap, NotifyResponseEnum, Response,
    },
    error::FeishuFailedRequestError,
    notify::feishu::{
        api_define::NotifyResponse as FeishuNotifyResponse, post::post as feishu_post,
    },
};

const ALI_CLOUD_MONITOR_HOST: &str = "https://cloudmonitornext.console.aliyun.com";

pub async fn handle_request(
    api_keys: Vec<AlertKeyMap>,
    body: AlertBody,
) -> Result<impl Reply, Rejection> {
    let mut results = Vec::new();
    for api_key in api_keys {
        if api_key.destination == AlertDestinations::Feishu {
            let template = match body {
                AlertBody::Threshold(ref body) => {
                    threshold_alert_state_to_feishu_template_color(&body.alert_state)
                }
                AlertBody::Event(ref body) => event_level_to_feishu_template_color(&body.level),
            };
            let title = match body {
                AlertBody::Threshold(ref body) => {
                    let instance_name = match body.instance_name == "null" {
                        true => "Instance".to_string(),
                        false => format!("{} instance", body.instance_name),
                    };
                    format!(
                        "{}({}) {} {}（{}）",
                        instance_name,
                        body.alert_name,
                        body.raw_metric_name,
                        body.alert_state,
                        body.cur_value,
                    )
                }
                AlertBody::Event(ref body) => body.name.to_string(),
            };

            let mut url = format!("{}/newalarm-happen/type:product", ALI_CLOUD_MONITOR_HOST);
            if let AlertBody::Threshold(ref body) = body {
                if let Some(group_id) = &body.group_id {
                    url = format!(
                        "{}/app-group/{}/alarmRule?ruleName={}",
                        ALI_CLOUD_MONITOR_HOST, group_id, body.alert_name
                    );
                }
            }

            let message = match body {
                AlertBody::Threshold(ref body) => format!(
                    "Instance: **{}**\nRule Id: {}\nRelated resource: {}\nStatus: **{}**\n{}: {}(**{}**)",
                    body.instance_name,
                    body.alert_name,
                    body.dimensions,
                    body.alert_state,
                    body.raw_metric_name,
                    body.expression,
                    body.cur_value,
                ),
                AlertBody::Event(ref body) => format!(
                    "**{}**\nInstance: {}\n- level: {}",
                    body.product, body.instance_name, body.level,
                ),
            };
            match feishu_post(api_key.key, title.clone(), url, message, Some(template)).await {
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

// POST /api/v1/alicloud_monitor/alerts?apiKey=<api-key>,<api-key>
// apiKey format: "feishu_<API_KEY>,feishu_<API_KEY>"
pub fn alert() -> BoxedFilter<(impl Reply,)> {
    let log = warp::log::custom(|info| {
        info!(
            "method: {}, path: {}, status: {}",
            info.method(),
            info.path(),
            info.status(),
        );
    });

    warp::post()
        .and(warp::path!("api" / "v1" / "alicloud_monitor" / "alerts"))
        .and(warp::body::content_length_limit(1024 * 1024 * 10))
        .and(check_api_key())
        .and(log_form())
        .and_then(handle_request)
        .with(log)
        .boxed()
}
