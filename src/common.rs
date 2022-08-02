use std::str::FromStr;
use std::string::ToString;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use warp::{Filter, Rejection};

use crate::error::ConversionError;

#[derive(Display, Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum AlertDestinations {
    Grafana,
    Feishu,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub api_key: String,
}

#[derive(Debug)]
pub struct AlertKeyMap {
    pub destination: AlertDestinations,
    pub key: String,
}

// {
// 	"Extra": null,
// 	"StatusCode": 0,
// 	"StatusMessage": "success"
// }
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FeishuAPISuccessResponse {
    pub extra: Option<String>,
    pub status_code: u16,
    pub status_message: String,
}
// {
// 	"code": 9499,
// 	"msg": "Bad Request",
// 	"data": {}
// }
#[derive(Debug, Deserialize, Serialize)]
pub struct FeishuAPIErrorResponse {
    pub code: u16,
    pub msg: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    Success,
    Failed,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FeishuAPIResponse {
    FeishuAPISuccessResponse(FeishuAPISuccessResponse),
    FeishuAPIErrorResponse(FeishuAPIErrorResponse),
}

#[derive(Debug, Serialize)]
pub struct AlertResponse {
    pub destination: String,
    pub status: AlertStatus,
    pub result: FeishuAPIResponse,
}

#[derive(Serialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
    pub data: Vec<AlertResponse>,
}

pub fn check_api_key() -> impl Filter<Extract = (Vec<AlertKeyMap>,), Error = Rejection> + Copy {
    warp::query::<QueryParams>().and_then(|query: QueryParams| async move {
        let api_key = query.api_key;

        api_key
            .split(',')
            .map(|s| {
                if !s.contains('_') {
                    return Err(warp::reject::custom(ConversionError));
                }

                let mut parts = s.split('_');
                let destination = parts.next().unwrap();
                let key = parts.next().unwrap();

                if let Ok(destination) = AlertDestinations::from_str(destination) {
                    Ok(AlertKeyMap {
                        destination,
                        key: key.to_string(),
                    })
                } else {
                    Err(warp::reject::custom(ConversionError))
                }
            })
            .collect()
    })
}

// pub fn log_json<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
//     is_content_type::<Json>()
//         .and(bytes())
//         .and_then(|buf| async move {
//             Json::decode(buf).map_err(|err| {
//                 // tracing::debug!("request json body error: {}", err);
//                 reject::known(BodyDeserializeError { cause: err })
//             })
//         })
// }
// pub fn log_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
//     warp::body::bytes()
//         .and(|body: bytes::Bytes| async move {
//             // info!("{}", String::from_utf8_lossy(&body));
//             // info!("bytes = {:?}", body);
//             // info!("Received body: {}", std::str::from_utf8(body).expect("Error converting bytes to &str"));
//             info!("Received body: {}", String::from_utf8_lossy(&body));
//             // serde_json::from_slice(&body).map_err(Into::into)
//             // .map_err(|err: E| {
//             //     // tracing::debug!("request json body error: {}", err);
//             //     info!("request json body error: {}", err);
//             //     // warp::reject::custom(BodyDeserializeError { cause: err })
//             //     warp::reject::custom(ConversionError)
//             // })
//         })
// }
