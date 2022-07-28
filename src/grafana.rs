use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct EvalMatches {
    pub value: i32,
    pub metric: String,
    pub tags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrafanaAlert {
    pub dashboard_id: i32,
    pub eval_matches: Vec<EvalMatches>,
    pub image_url: String,
    pub message: String,
    pub org_id: i32,
    pub panel_id: i32,
    pub rule_id: i32,
    pub rule_name: String,
    pub rule_url: String,
    pub state: String,
    pub tags: HashMap<String, String>,
    pub title: String,
}
