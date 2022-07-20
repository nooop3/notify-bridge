use std::str::FromStr;
use std::string::ToString;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use warp::{reject::Reject, Filter, Rejection};

#[derive(Display, Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum AlertDestination {
    Grafana,
    Feishu,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyParams {
    pub api_key: String,
}

#[derive(Debug)]
pub struct AlertKeyMap {
    pub alert_destination: AlertDestination,
    pub key: String,
}

#[derive(Debug, Serialize)]
pub struct AlertResonse {
    pub status: String,
    pub destination: String,
}

#[derive(Serialize)]
pub struct Response {
    pub status: String,
    pub items: Vec<AlertResonse>,
}

#[derive(Debug)]
pub struct ConversionError;
impl Reject for ConversionError {}

pub fn check_api_key() -> impl Filter<Extract = (Vec<AlertKeyMap>,), Error = Rejection> + Copy {
    warp::query::<ApiKeyParams>().and_then(|param: ApiKeyParams| async move {
        let api_key = param.api_key;

        api_key
            .split(',')
            .map(|s| {
                if !s.contains('_') {
                    return Err(warp::reject::custom(ConversionError));
                }

                let mut parts = s.split('_');
                let destination = parts.next().unwrap();
                let key = parts.next().unwrap();

                if let Ok(destination) = AlertDestination::from_str(destination) {
                    Ok(AlertKeyMap {
                        alert_destination: destination,
                        key: key.to_string(),
                    })
                } else {
                    Err(warp::reject::custom(ConversionError))
                }
            })
            .collect()
    })
}
