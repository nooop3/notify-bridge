use serde::{Deserialize, Serialize};
use serde_json::Value;

const fn default_true() -> bool {
    true
}
const fn default_false() -> bool {
    false
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Tag {
    PlainText,
    Img,
    Button,
    Div,
    Hr,
    Action,
    Note,
    Markdown,
    Interactive,
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
pub struct CardTitle {
    // pub tag: Tag::PlainText,
    pub tag: Tag,
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
pub struct CardHeader {
    pub title: CardTitle,
    pub template: TemplateColor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardTextTag {
    PlainText,
    LarkMd,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardText {
    tag: CardTextTag,
    content: String,
    lines: Option<i32>,
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
    // pub tag: Tag::Img,
    tag: Tag,
    img_key: String,
    alt: CardText,
    preview: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardImage {
    // pub tag: Tag::Img,
    tag: Tag,
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
pub struct CardPlainText {
    // pub tag: Tag::PlainText,
    tag: Tag,
    content: String,
    lines: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardConfirmText {
    pub title: CardPlainText,
    pub text: CardPlainText,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardConfirm {
    pub title: CardConfirmText,
    pub text: CardConfirmText,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardButton {
    // pub tag: Tag::Button,
    pub tag: Tag,
    pub text: CardText,
    pub url: Option<String>,
    pub multi_url: Option<CardMultiUrl>,
    pub r#type: Option<CardButtonType>,
    pub value: Option<Value>,
    pub confirm: Option<CardConfirm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardExtra {
    Image(CardImage),
    Button(Box<CardButton>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardDivModule {
    // pub tag: Tag::Div,
    pub tag: Tag,
    pub text: Option<CardText>,
    pub fields: Option<Vec<CardField>>,
    pub extra: Option<CardExtra>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardHrModule {
    // pub tag: Tag::Hr,
    pub tag: Tag,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardImageModule {
    // pub tag: Tag::Img,
    pub tag: Tag,
    pub img: CardImage,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct CardActionModule {
    // pub tag: Tag::Action,
    pub tag: Tag,
    pub actions: Vec<ActionElement>,
    pub layout: Option<ActionLayout>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NoteElement {
    Text(CardText),
    Image(CardImageElement),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardNoteModule {
    // pub tag: Tag::Note,
    pub tag: Tag,
    pub elements: Vec<NoteElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardMarkdownModule {
    // pub tag: Tag::Markdown,
    pub tag: Tag,
    pub content: String,
    pub href: Option<CardHref>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Element {
    Div(Box<CardDivModule>),
    Hr(CardHrModule),
    Image(CardImageModule),
    Note(CardNoteModule),
    Markdown(CardMarkdownModule),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementI18n {
    pub en_us: Vec<Element>,
    pub zh_cn: Vec<Element>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub config: CardConfig,
    pub header: CardHeader,
    pub elements: Option<Vec<Element>>,
    pub i18n_elements: ElementI18n,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    // Tag::Internactive,
    pub msg_type: Tag,
    pub card: Card,
}
