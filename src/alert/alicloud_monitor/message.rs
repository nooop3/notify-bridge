use serde::Deserializer;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ThresholdAlertState {
    OK,
    Alert,
    InsufficientData,
}

#[derive(Display, Debug, Deserialize, Serialize, PartialEq, Eq, EnumString)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum ThresholdTriggerLevel {
    Critical,
    Warning,
    Info,
    #[serde(rename = "null")]
    Unknown,
}

fn deserialize_percentage<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(percent_encoding::percent_decode_str(&s)
        .decode_utf8_lossy()
        .to_string())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdAlertBody {
    pub alert_name: String,
    pub alert_state: ThresholdAlertState,
    pub cur_value: String,
    #[serde(deserialize_with = "deserialize_percentage")]
    pub dimensions: String,
    #[serde(deserialize_with = "deserialize_percentage")]
    pub expression: String,
    pub instance_name: String,
    #[serde(deserialize_with = "deserialize_percentage")]
    pub metric_name: String,
    pub metric_project: String,
    pub namespace: String,
    pub pre_trigger_level: ThresholdTriggerLevel,
    pub rule_id: String,
    pub timestamp: String,
    pub trigger_level: ThresholdTriggerLevel,
    pub user_id: String,
    pub group_id: Option<String>,
}

// {
//   "traceId": "b0eaeed6-6758-4d45-ac64-c52437de****",
//   "resourceId": "acs:ecs:cn-hangzhou:145394352104****:snapshot/s-bp13s5zbbwxm780t****",
//   "ver": "1.0",
//   "product": "ECS",
//   "instanceName": "s-bp13s5zbbwxm780t****",
//   "level": "INFO",
//   "userId": "145394352104****",
//   "content": {
//     "result": "accomplished",
//     "snapshotId": "s-bp13s5zbbwxm780t****",
//     "snapshotType": "timer",
//     "snapshotName": "auto2.0_20210224_sp-bp1etszs074zjp4p****",
//     "diskId": "d-bp1ioh7hw6l94rys****",
//     "startTime": "2021-02-23T17:05:13Z",
//     "endTime": "2021-02-23T17:11:13Z"
//   },
//   "regionId": "cn-hangzhou",
//   "eventTime": "20210224T011113.709+0800",
//   "name": "Snapshot:CreateSnapshotCompleted",
//   "id": "103E55FC-7FC3-4B3D-AE12-C19EA84C****",
//   "status": "Normal"
// }
#[derive(Debug, Deserialize, Serialize)]
pub enum EventLevel {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventContent {
    pub result: String,
    pub snapshot_id: String,
    pub snapshot_type: String,
    pub snapshot_name: String,
    pub disk_id: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAlertBody {
    pub trace_id: String,
    pub resource_id: String,
    pub ver: String,
    pub product: String,
    pub instance_name: String,
    pub level: EventLevel,
    pub user_id: String,
    pub content: EventContent,
    pub region_id: String,
    pub event_time: String,
    pub name: String,
    pub id: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AlertBody {
    Threshold(ThresholdAlertBody),
    Event(EventAlertBody),
}
