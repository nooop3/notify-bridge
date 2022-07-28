use crate::{
    common::{AlertStatus, FeishuAPIResponse},
    notify::feishu::card::{
        Card, CardHeader, CardTitle, Message, Module, NoteElement, StringI18n, TemplateColor,
        TextElement,
    },
};

use reqwest::Client;

const FEISHU_OPEN_API_PREFIX: &str = "https://open.feishu.cn/open-apis/bot/v2/hook/";

pub fn notify() -> Message {
    let title = "Alert";

    let config = super::card::CardConfig {
        enable_forward: true,
        update_multi: false,
    };
    let header = CardHeader {
        title: CardTitle::PlainText(StringI18n {
            en_us: title.to_string(),
            zh_cn: title.to_string(),
        }),
        template: TemplateColor::Success,
    };

    let note = Module::Note {
        elements: vec![NoteElement::PlainText(TextElement {
            content: "Note: You may need related permissions to open the buttons above."
                .to_string(),
            lines: None,
        })],
    };

    let card = Card {
        config,
        header,
        i18n_elements: None,
        elements: Some(vec![note]),
        // i18n_elements: Some(ElementI18n {
        //     en_us: vec![Module::Note(note)],
        //     zh_cn: vec![],
        // }),
    };

    Message::Interactive(card)
}

pub async fn post(
    api_key: String,
) -> Result<(AlertStatus, FeishuAPIResponse), Box<dyn std::error::Error>> {
    let message = notify();
    info!("{}", serde_json::to_string(&message).unwrap());

    let client = Client::new();
    let response = client
        .post(&format!("{}{}", FEISHU_OPEN_API_PREFIX, api_key))
        .json(&message)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let response: FeishuAPIResponse = response.json().await?;
            let status = match response {
                FeishuAPIResponse::FeishuAPISuccessResponse(_) => AlertStatus::Success,
                FeishuAPIResponse::FeishuAPIErrorResponse(_) => AlertStatus::Failed,
            };
            Ok((status, response))
        }
        status_code => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to send Feishu API request, status: {}", status_code),
        ))),
    }
}
