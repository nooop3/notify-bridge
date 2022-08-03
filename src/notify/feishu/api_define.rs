use serde::{Deserialize, Serialize};

use crate::common::AlertStatus;

// {
// 	"Extra": null,
// 	"StatusCode": 0,
// 	"StatusMessage": "success"
// }
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct APISuccessResponse {
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
pub struct APIErrorResponse {
    pub code: u16,
    pub msg: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum APIResponse {
    APISuccessResponse(APISuccessResponse),
    APIErrorResponse(APIErrorResponse),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotifyResponse {
    pub destination: String,
    pub status: AlertStatus,
    pub result: APIResponse,
}
