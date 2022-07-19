use std::str::FromStr;

use serde::Deserialize;
use strum::EnumString;

#[derive(Debug, PartialEq, Eq, EnumString)]
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
