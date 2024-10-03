use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{property::DatePropertyValue, user::User};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichText {
    #[default]
    None,
    Equation {
        equation: Equation,
        annotations: Annotations,
        plain_text: String,
        href: Option<String>,
    },
    Mention {
        mention: Mention,
        annotations: Annotations,
        plain_text: String,
        href: Option<String>,
    },
    Text {
        text: Text,
        annotations: Option<Annotations>,
        plain_text: Option<String>,
        href: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Equation {
    pub expression: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Text {
    pub content: String,
    pub link: Option<Link>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextColor {
    #[default]
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    GreenBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Link {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Mention {
    Database { database: DatabaseMention },
    Date { date: DatePropertyValue },
    LinkPreview { link_preview: LinkPreviewMention },
    LinkMention { link_mention: LinkMentionMention },
    TemplateMention { template_mention: TemplateMention },
    Page { page: PageMention },
    User { user: User },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DatabaseMention {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkPreviewMention {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkMentionMention {
    pub description: Option<String>,
    pub href: Option<String>,
    pub icon_url: Option<String>,
    pub iframe_url: Option<String>,
    pub link_author: Option<String>,
    pub padding: Option<u32>,
    pub thumbnail_url: Option<String>,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct PageMention {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TemplateMention {
    TemplateMentionDate {
        template_mention_date: TemplateMentionDate,
    },
    TemplateMentionUser {
        template_mention_user: TemplateMentionUser,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TemplateMentionDate {
    Today,
    Now,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TemplateMentionUser {
    Me,
}

impl RichText {
    pub fn plain_text(&self) -> Option<String> {
        match self {
            RichText::None => None,
            RichText::Equation { plain_text, .. } => Some(plain_text.clone()),
            RichText::Mention { plain_text, .. } => Some(plain_text.clone()),
            RichText::Text { plain_text, .. } => plain_text.clone(),
        }
    }
}
