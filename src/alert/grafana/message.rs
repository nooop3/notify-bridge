use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::Display;

/*
{
  "dashboardId": 1,
  "evalMatches": [
    {
      "value": 1,
      "metric": "Count",
      "tags": {}
    }
  ],
  "imageUrl": "https://grafana.com/assets/img/blog/mixed_styles.png",
  "message": "Notification Message",
  "orgId": 1,
  "panelId": 2,
  "ruleId": 1,
  "ruleName": "Panel Title alert",
  "ruleUrl": "http://localhost:3000/d/hZ7BuVbWz/test-dashboard?fullscreenu0026editu0026tab=alertu0026panelId=2u0026orgId=1",
  "state": "alerting",
  "tags": {
    "tag name": "tag value"
  },
  "title": "[Alerting] Panel Title alert"
}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct EvalMatches {
    pub value: f64,
    pub metric: String,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum AlertState {
    NoData,
    Paused,
    Alerting,
    #[serde(rename = "ok")]
    OK,
    Pending,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertBody {
    pub title: String,
    pub rule_id: i32,
    pub rule_name: String,
    pub state: AlertState,
    pub eval_matches: Vec<EvalMatches>,
    pub org_id: i32,
    pub dashboard_id: i32,
    pub panel_id: i32,
    pub tags: HashMap<String, String>,
    pub rule_url: Option<String>,
    pub image_url: Option<String>,
    pub message: Option<String>,
}
