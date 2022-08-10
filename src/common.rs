use std::str::FromStr;
use std::string::ToString;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use warp::{Filter, Rejection};

use crate::{
    alert::alicloud_monitor::message::AlertBody,
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

pub fn log_form() -> impl Filter<Extract = (AlertBody,), Error = Rejection> + Copy {
    warp::body::bytes().and_then(|buf: bytes::Bytes| async move {
        let body = std::str::from_utf8(&buf).unwrap();
        info!("Received request form: {}", body);

        serde_urlencoded::from_str::<AlertBody>(body).map_err(|err| {
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

// pub fn log_body() -> impl Filter<Extract = (), Error = Rejection> + Copy {
//     warp::body::bytes()
//         .map(move |b: Bytes| {
//             info!(
//                 "Request body: {}",
//                 std::str::from_utf8(&b).expect("error converting bytes to &str")
//             );
//         })
//         .untuple_one()
// }

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
