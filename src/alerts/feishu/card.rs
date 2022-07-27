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
pub struct PlainText;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Img;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Button;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Div;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Hr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Note;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Markdown;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Internactive;

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
    pub tag: PlainText,
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
    tag: Img,
    img_key: String,
    alt: CardText,
    preview: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardImage {
    tag: Img,
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
    tag: PlainText,
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
    pub tag: Button,
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
    pub tag: Div,
    pub text: Option<CardText>,
    pub fields: Option<Vec<CardField>>,
    pub extra: Option<CardExtra>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardHrModule {
    pub tag: Hr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardImageModule {
    pub tag: Img,
    pub img: CardImage,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NoteElement {
    Text(CardText),
    Image(CardImageElement),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardNoteModule {
    pub tag: Note,
    pub elements: Vec<NoteElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardMarkdownModule {
    pub tag: Markdown,
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
    pub msg_type: Internactive,
    pub card: Card,
}
