use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Number;

use super::{file::File, rich_text::RichText, user::User};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Property {
    Title {
        id: String,
        title: RichText,
    },
    RichText {
        id: String,
        rich_text: RichText,
    },
    Number {
        number: Number,
    },
    Select {
        id: String,
        name: String,
        color: Color,
    },
    Status {
        id: String,
        name: String,
        color: Color,
    },
    MultiSelect {
        id: String,
        name: String,
        color: Color,
    },
    Date {
        id: String,
        date: DatePropertyValue,
    },
    Formula {
        id: String,
        formula: FormulaPropertyValue,
    },
    Relation {
        id: String,
        relation: RelationPropertyValue,
    },
    Rollup {
        id: String,
        rollup: RollupPropertyValue,
    },
    People {
        id: String,
        people: Vec<User>,
    },
    Files {
        id: String,
        files: Vec<FilePropertyValue>,
    },
    Checkbox {
        id: String,
        checkbox: bool,
    },
    Url {
        id: String,
        url: String,
    },
    Email {
        id: String,
        email: String,
    },
    PhoneNumber {
        id: String,
        phone_number: String,
    },
    CreatedTime {
        id: String,
        created_time: DateTime<Utc>,
    },
    CreatedBy {
        id: String,
        created_by: User,
    },
    LastEditedTime {
        id: String,
        last_edited_time: DateTime<Utc>,
    },
    LastEditedBy {
        id: String,
        last_edited_by: User,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Default,
    Gray,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DatePropertyValue {
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub time_zone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormulaPropertyValue {
    String { string: Option<String> },
    Number { number: Option<Number> },
    Boolean { boolean: bool },
    Date { date: Option<DatePropertyValue> },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RelationPropertyValue {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RollupPropertyValue {
    String { string: Option<String> },
    Number { number: Number },
    Date { date: DateTime<Utc> },
    Array { results: Vec<RollupPropertyValue> },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FilePropertyValue {
    pub name: String,
    #[serde(flatten)]
    pub file: File,
}
