use crate::{
    common::{AlertStatus, FeishuAPIResponse},
    notify::feishu::card::{
        ActionElement, ActionLayout, Card, CardButton, CardButtonType, CardHeader, CardText,
        CardTitle, CardTitlePlainText, Message, Module, NoteElement, TemplateColor, TextElement,
    },
};

use reqwest::Client;

const FEISHU_OPEN_API_PREFIX: &str = "https://open.feishu.cn/open-apis/bot/v2/hook/";

pub fn notify(
    title: String,
    url: String,
    content: String,
    template: Option<TemplateColor>,
) -> Message {
    let config = super::card::CardConfig {
        enable_forward: true,
        update_multi: false,
    };
    let header = CardHeader {
        title: CardTitle::PlainText(CardTitlePlainText::PlainText(TextElement {
            content: title,
            ..Default::default()
        })),
        template: template.unwrap_or(TemplateColor::Blue),
    };

    let div = Module::Div {
        text: Some(CardText::LarkMd(TextElement {
            content,
            ..Default::default()
        })),
        fields: None,
        extra: None,
    };

    let hr = Module::Hr;

    let action = Module::Action {
        actions: vec![ActionElement::Button(CardButton {
            text: CardText::LarkMd(TextElement {
                content: "View".to_string(),
                ..Default::default()
            }),
            url: Some(url),
            multi_url: None,
            r#type: Some(CardButtonType::Primary),
            value: None,
            confirm: None,
        })],
        layout: Some(ActionLayout::Flow),
    };

    let note = Module::Note {
        elements: vec![NoteElement::PlainText(TextElement {
            content: "Note: You may need related permissions to open the buttons above."
                .to_string(),
            ..Default::default()
        })],
    };

    let card = Card {
        config,
        header,
        i18n_elements: None,
        elements: Some(vec![div, hr, action, note]),
    };

    Message::Interactive(card)
}

pub async fn post(
    api_key: String,
    title: String,
    url: String,
    content: String,
    template: Option<TemplateColor>,
) -> Result<(AlertStatus, FeishuAPIResponse), Box<dyn std::error::Error>> {
    let message = notify(title, url, content, template);
    info!(
        "Notify Feishu: {}",
        serde_json::to_string(&message).unwrap()
    );

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
