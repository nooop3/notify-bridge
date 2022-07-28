use serde::{Deserialize, Serialize};
use serde_json::Value;

const fn default_true() -> bool {
    true
}
const fn default_false() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardConfig {
    #[serde(default = "default_true")]
    pub enable_forward: bool,
    #[serde(default = "default_false")]
    pub update_multi: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringI18n {
    pub en_us: String,
    pub zh_cn: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "tag", content = "i18n")]
pub enum CardTitle {
    PlainText(StringI18n),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateColor {
    Green,
    #[serde(rename = "green")]
    Success,
    #[serde(rename = "green")]
    Completed,

    Orange,
    #[serde(rename = "orange")]
    Warning,
    #[serde(rename = "orange")]
    Notify,

    Red,
    #[serde(rename = "red")]
    Error,
    #[serde(rename = "red")]
    Failed,

    Grey,
    #[serde(rename = "grey")]
    Disabled,
    #[serde(rename = "grey")]
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
pub struct CardHeader {
    pub title: CardTitle,
    pub template: TemplateColor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextElement {
    pub content: String,
    pub lines: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
pub enum CardText {
    PlainText(TextElement),
    LarkMd(TextElement),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardField {
    pub is_short: bool,
    pub text: CardText,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardImageMode {
    FitHorizontal,
    CropCenter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardImageElement {
    img_key: String,
    alt: CardText,
    preview: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardImage {
    img_key: String,
    alt: CardText,
    title: Option<CardText>,
    custom_width: Option<i32>,
    #[serde(default = "default_false")]
    compact_width: bool,
    mode: Option<CardImageMode>,
    preview: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardMultiUrl {
    pub url: String,
    pub andriod_url: String,
    pub ios_url: String,
    pub pc_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardHref {
    pub url_val: CardMultiUrl,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardButtonType {
    Default,
    Primary,
    Danger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardConfirm {
    // pub title: CardText::PlainText(TextElement),
    pub title: CardText,
    pub text: CardText,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardButton {
    pub text: CardText,
    pub url: Option<String>,
    pub multi_url: Option<CardMultiUrl>,
    pub r#type: Option<CardButtonType>,
    pub value: Option<Value>,
    pub confirm: Option<CardConfirm>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
pub enum CardExtra {
    Img(CardImage),
    Button(Box<CardButton>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
pub enum ActionElement {
    Button(CardButton),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionLayout {
    Bisected,
    Trisection,
    Flow,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
pub enum NoteElement {
    PlainText(TextElement),
    LarkMd(TextElement),
    Img(CardImageElement),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
pub enum Module {
    Div {
        text: Option<CardText>,
        fields: Option<Vec<CardField>>,
        extra: Option<CardExtra>,
    },
    Hr,
    Img {
        img: CardImage,
    },
    Action {
        actions: Vec<ActionElement>,
        layout: Option<ActionLayout>,
    },
    Note {
        elements: Vec<NoteElement>,
    },
    Markdown {
        content: String,
        href: Option<CardHref>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementI18n {
    pub en_us: Vec<Module>,
    pub zh_cn: Vec<Module>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub config: CardConfig,
    pub header: CardHeader,
    pub elements: Option<Vec<Module>>,
    pub i18n_elements: Option<ElementI18n>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "msg_type", content = "card")]
pub enum Message {
    Interactive(Card),
}
