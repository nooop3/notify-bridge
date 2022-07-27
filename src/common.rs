use std::str::FromStr;
use std::string::ToString;

use serde::{Deserialize, Serialize};
use serde_json::Value;
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
const fn default_false() -> bool {
    false
}
fn plain_text_string() -> String {
    "plain_text".to_string()
}
fn img_string() -> String {
    "img".to_string()
}
fn button_string() -> String {
    "button".to_string()
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
    Green,
    #[serde(alias = "green")]
    Success,
    #[serde(alias = "green")]
    Completed,

    Orange,
    #[serde(alias = "orange")]
    Warning,
    #[serde(alias = "orange")]
    Notify,

    Red,
    #[serde(alias = "red")]
    Error,
    #[serde(alias = "red")]
    Failed,

    Grey,
    #[serde(alias = "grey")]
    Disabled,
    #[serde(alias = "grey")]
    Invalid,

    Blue,
    Wathet,
    Turquoise,
    Yellow,
    Carmine,
    Violet,
    Purple,
    Indigo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageHeader {
    pub title: FeishuAlertMessageTitle,
    pub template: TemplateColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageElementDivTag;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageElementLarkMdTag;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageElementHrTag;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageElementImgTag;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageElementNoteTag;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeishuAlertMessageElementTag {
    Div(FeishuAlertMessageElementDivTag),
    Markdown(FeishuAlertMessageElementLarkMdTag),
    Hr(FeishuAlertMessageElementHrTag),
    Img(FeishuAlertMessageElementImgTag),
    Note(FeishuAlertMessageElementNoteTag),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageElement2 {
    pub tag: FeishuAlertMessageElementTag,
    pub fields: Vec<FeishuAlertMessageField>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeishuAlertMessageTextTag {
    PlainText,
    LarkMd,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageText {
    tag: FeishuAlertMessageTextTag,
    content: String,
    lines: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageField {
    pub is_short: bool,
    pub text: FeishuAlertMessageText,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeishuAlertMessageImgMode {
    FitHorizontal,
    CropCenter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageImg {
    #[serde(default = "img_string")]
    tag: String,
    img_key: String,
    alt: FeishuAlertMessageText,
    title: Option<FeishuAlertMessageText>,
    custom_width: Option<i32>,
    #[serde(default = "default_false")]
    compact_width: bool,
    mode: Option<FeishuAlertMessageImgMode>,
    preview: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageMultiUrl {
    pub url: String,
    pub andriod_url: String,
    pub ios_url: String,
    pub pc_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeishuAlertMessageHref {
    pub url_val: FeishuAlertMessageMultiUrl,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageConfirm {
    pub title: FeishuAlertMessageText,
    pub text: FeishuAlertMessageText,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeishuAlertMessageButtonType {
    Default,
    Primary,
    Danger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageButton {
    #[serde(default = "button_string")]
    pub tag: String,
    pub text: FeishuAlertMessageText,
    pub url: Option<String>,
    pub multi_url: Option<FeishuAlertMessageMultiUrl>,
    pub r#type: Option<FeishuAlertMessageButtonType>,
    pub value: Option<Value>,
    pub confirm: Option<FeishuAlertMessageConfirm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FeishuAlertMessageExtra {
    Image(FeishuAlertMessageImg),
    Button(FeishuAlertMessageButton),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageDivModule {
    pub tag: FeishuAlertMessageElementDivTag,
    pub text: FeishuAlertMessageText,
    pub fields: Option<Vec<FeishuAlertMessageField>>,
    pub extra: Option<FeishuAlertMessageExtra>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageMarkdownModule {
    pub tag: FeishuAlertMessageElementLarkMdTag,
    pub content: String,
    pub href: Option<FeishuAlertMessageHref>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageHrModule {
    pub tag: FeishuAlertMessageElementHrTag,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageImgModule {
    pub tag: FeishuAlertMessageElementImgTag,
    pub img: FeishuAlertMessageImg,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FeishuAlertMessageNoteElement {
    Text(FeishuAlertMessageText),
    Img(FeishuAlertMessageImg),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessageNoteModule {
    pub tag: FeishuAlertMessageElementNoteTag,
    pub elements: Vec<FeishuAlertMessageNoteElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FeishuAlertMessageModule {
    Div(Box<FeishuAlertMessageDivModule>),
    Markdown(FeishuAlertMessageMarkdownModule),
    Hr(FeishuAlertMessageHrModule),
    Img(FeishuAlertMessageImgModule),
    Note(FeishuAlertMessageNoteModule),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementI18n {
    pub en_us: Vec<FeishuAlertMessageModule>,
    pub zh_cn: Vec<FeishuAlertMessageModule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuAlertMessage {
    pub config: FeishuAlertMessageConfig,
    pub header: FeishuAlertMessageHeader,
    pub elements: Option<Vec<FeishuAlertMessageModule>>,
    pub i18n_elements: ElementI18n,
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
