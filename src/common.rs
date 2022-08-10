use std::str::FromStr;
use std::string::ToString;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use strum::{Display, EnumString};
use warp::{Filter, Rejection};

use crate::{
    error::{ConversionError, FormBodyDeserializeError},
    notify::feishu::api_define::NotifyResponse as FeishuNotifyResponse,
};

#[derive(Display, Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum AlertDestinations {
    Feishu,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub api_key: String,
}

#[derive(Debug)]
pub struct AlertKeyMap {
    pub destination: AlertDestinations,
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    Success,
    Failed,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NotifyResponseEnum {
    Feishu(FeishuNotifyResponse),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
    pub data: Vec<NotifyResponseEnum>,
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

pub fn log_json<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    warp::body::bytes().and_then(|body: bytes::Bytes| async move {
        let body = std::str::from_utf8(&body).unwrap();
        let json_body = match json::parse(body) {
            Ok(json) => json,
            Err(err) => {
                return Err(warp::reject::custom(FormBodyDeserializeError {
                    message: format!("Can not parse body to JSON: {}", err),
                }));
            }
        };
        info!("Received request json: {}", json_body);

        serde_json::from_str(body).map_err(|err| {
            warp::reject::custom(FormBodyDeserializeError {
                message: err.to_string(),
            })
        })
    })
}

pub fn log_form<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    warp::body::bytes().and_then(|buf: bytes::Bytes| async move {
        let body = std::str::from_utf8(&buf).unwrap();
        info!("Received request form: {}", body);

        serde_urlencoded::from_str::<T>(body).map_err(|err| {
            warp::reject::custom(FormBodyDeserializeError {
                message: err.to_string(),
            })
        })
    })
}

// pub fn log<F>(func: F) -> warp::log::Log<F> {
//     warp::log::custom(|info| {
//         info!("method: {}, path: {}, status: {}", info.method(), info.path(), info.status());
//     })
// }
