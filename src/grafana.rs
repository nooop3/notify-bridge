use std::collections::HashMap;

use serde::{Serialize, Deserialize};

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
struct EvalMatches {
    value: i32,
    metric: String,
    tags: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrafanaAlert {
    dashboard_id: i32,
    eval_matches: Vec<EvalMatches>,
    image_url: String,
    message: String,
    org_id: i32,
    panel_id: i32,
    rule_id: i32,
    rule_name: String,
    rule_url: String,
    state: String,
    tags: HashMap<String, String>,
    title: String,
}
