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

#[derive(Debug, Serialize)]
pub struct AlertResonse {
    pub destination: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
    pub data: Vec<AlertResonse>,
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
