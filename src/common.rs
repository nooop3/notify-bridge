use std::str::FromStr;
use std::string::ToString;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

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

pub fn parse_str(s: &str) -> Vec<AlertKeyMap> {
    s.split(',')
        .filter_map(|s| match s.contains('_') {
            true => {
                let mut parts = s.split('_');
                Some(AlertKeyMap {
                    alert_destination: match AlertDestination::from_str(parts.next().unwrap()) {
                        Ok(v) => v,
                        Err(_) => panic!("AlertDestination::from_str error"),
                    },
                    key: parts.next().unwrap().to_string(),
                })
            }
            false => None,
        })
        .collect()
}
