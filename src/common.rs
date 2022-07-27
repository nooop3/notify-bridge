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

// {
//   "config": {
//     "wide_screen_mode": true
//   },
//   "header": {
//     "title": {
//       "tag": "plain_text",
//       "content": "你有一个休假申请待审批"
//     }
//   },
//   "elements": [
//     {
//       "tag": "div",
//       "fields": [
//         {
//           "is_short": true,
//           "text": {
//             "tag": "lark_md",
//             "content": "**申请人**\n王晓磊"
//           }
//         },
//         {
//           "is_short": true,
//           "text": {
//             "tag": "lark_md",
//             "content": "**休假类型：**\n年假"
//           }
//         },
//         {
//           "is_short": false,
//           "text": {
//             "tag": "lark_md",
//             "content": ""
//           }
//         },
//         {
//           "is_short": false,
//           "text": {
//             "tag": "lark_md",
//             "content": "**时间：**\n2020-4-8 至 2020-4-10（共3天）"
//           }
//         },
//         {
//           "is_short": false,
//           "text": {
//             "tag": "lark_md",
//             "content": ""
//           }
//         },
//         {
//           "is_short": true,
//           "text": {
//             "tag": "lark_md",
//             "content": "**备注**\n因家中有急事，需往返老家，故请假"
//           }
//         }
//       ]
//     },
//     {
//       "tag": "note",
//       "elements": [
//         {
//           "tag": "img",
//           "img_key": "img_b79d3e29-5ffe-4897-822a-9743fe9f7b1g",
//           "alt": {
//             "tag": "plain_text",
//             "content": "图片"
//           }
//         },
//         {
//           "tag": "plain_text",
//           "content": "已同意休假申请"
//         }
//       ]
//     }
//   ]
// }

const fn default_true() -> bool {
    true
}
fn plain_text_string() -> String {
    "plain_text".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageConfig {
    #[serde(default = "default_true")]
    pub enable_forward: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringI18n {
    pub en_us: String,
    pub zh_cn: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageTitle {
    #[serde(default = "plain_text_string")]
    pub tag: String,
    pub i18n: StringI18n,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateColor {
    Blue,
    Wathet,
    Turquoise,
    Green,
    Yellow,
    Orange,
    Red,
    Carmine,
    Violet,
    Purple,
    Indigo,
    Grey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageHeader {
    pub title: FeishuAlertMessageTitle,
    pub template: TemplateColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageText {
    pub tag: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageField {
    pub is_short: bool,
    pub text: FeishuAlertMessageText,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageElement {
    pub tag: String,
    pub fields: Vec<FeishuAlertMessageField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessage {
    pub config: FeishuAlertMessageConfig,
    pub header: FeishuAlertMessageHeader,
    pub elements: Vec<FeishuAlertMessageElement>,
}

// {
// 	"Extra": null,
// 	"StatusCode": 0,
// 	"StatusMessage": "success"
// }
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FeishuAPISuccessResponse {
    pub extra: Option<String>,
    pub status_code: u16,
    pub status_message: String,
}
// {
// 	"code": 9499,
// 	"msg": "Bad Request",
// 	"data": {}
// }
#[derive(Debug, Deserialize, Serialize)]
pub struct FeishuAPIErrorResponse {
    pub code: u16,
    pub msg: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    Success,
    Failed,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FeishuAPIResponse {
    FeishuAPISuccessResponse(FeishuAPISuccessResponse),
    FeishuAPIErrorResponse(FeishuAPIErrorResponse),
}

#[derive(Debug, Serialize)]
pub struct AlertResponse {
    pub destination: String,
    pub status: AlertStatus,
    pub result: FeishuAPIResponse,
}

#[derive(Serialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
    pub data: Vec<AlertResponse>,
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
