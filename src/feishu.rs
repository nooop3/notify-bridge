// extern crate reqwest;
// curl -X POST -H "Content-Type: application/json" \
//   -d '{"msg_type":"text","content":{"text":"request example"}}' \
//   https://open.feishu.cn/open-apis/bot/v2/hook/xxxxxxxxxxxxxxxxx

use crate::common::{AlertStatus, FeishuAPIResponse};
use reqwest::{header, Client};

const FEISHU_OPEN_API_PREFIX: &str = "https://open.feishu.cn/pen-apis/bot/v2/hook/";

pub async fn post_feishu_alert(
    api_key: String,
) -> Result<(AlertStatus, FeishuAPIResponse), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );

    let mut body = String::new();
    body.push_str("{\"msg_type\":\"text\",\"content\":{\"text\":\"request example\"}}");

    let client = Client::new();
    let response = client
        .post(&format!("{}{}", FEISHU_OPEN_API_PREFIX, api_key))
        .headers(headers)
        .body(body)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response: FeishuAPIResponse = response.json().await?;
            let status = match response {
                FeishuAPIResponse::FeishuAPISuccessResponse(_) => AlertStatus::Success,
                FeishuAPIResponse::FeishuAPIErrorResponse(_) => AlertStatus::Failed,
            };
            Ok((status, response))
        }
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to send Feishu API request",
        ))),
    }
}
