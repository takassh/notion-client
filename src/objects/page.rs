use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use serde_with::skip_serializing_none;

use super::{emoji::Emoji, file::File, parent::Parent, rich_text::RichText, user::User};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Page {
    pub id: String,
    pub created_time: DateTime<Utc>,
    pub created_by: User,
    pub last_edited_time: DateTime<Utc>,
    pub last_edited_by: User,
    pub archived: bool,
    pub icon: Option<Icon>,
    pub cover: Option<File>,
    pub properties: HashMap<String, PageProperty>,
    pub parent: Parent,
    pub url: String,
    pub public_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Icon {
    File(File),
    Emoji(Emoji),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PageProperty {
    Checkbox {
        id: Option<String>,
        checkbox: bool,
    },
    CreatedBy {
        id: Option<String>,
        created_by: User,
    },
    CreatedTime {
        id: Option<String>,
        created_time: DateTime<Utc>,
    },
    Date {
        id: Option<String>,
        date: Option<DatePropertyValue>,
    },
    Email {
        id: Option<String>,
        email: Option<String>,
    },
    Files {
        id: Option<String>,
        files: Vec<FilePropertyValue>,
    },
    Formula {
        id: Option<String>,
        formula: Option<FormulaPropertyValue>,
    },
    LastEditedBy {
        id: Option<String>,
        last_edited_by: User,
    },
    LastEditedTime {
        id: Option<String>,
        last_edited_time: Option<DateTime<Utc>>,
    },
    MultiSelect {
        id: Option<String>,
        multi_select: Vec<SelectPropertyValue>,
    },
    Number {
        id: Option<String>,
        number: Option<Number>,
    },
    People {
        id: Option<String>,
        people: Vec<User>,
    },
    PhoneNumber {
        id: Option<String>,
        phone_number: Option<String>,
    },
    Relation {
        id: Option<String>,
        relation: Vec<RelationPropertyValue>,
        has_more: Option<bool>,
    },
    Rollup {
        id: Option<String>,
        rollup: Option<RollupPropertyValue>,
    },
    RichText {
        id: Option<String>,
        rich_text: Vec<RichText>,
    },
    Select {
        id: Option<String>,
        select: Option<SelectPropertyValue>,
    },
    Status {
        id: Option<String>,
        status: Option<SelectPropertyValue>,
    },
    Title {
        id: Option<String>,
        title: Vec<RichText>,
    },
    Url {
        id: Option<String>,
        url: Option<String>,
    },
    #[serde(rename = "unique_id")]
    UniqueID {
        id: Option<String>,
        unique_id: Option<UniqueIDPropertyValue>,
    },
    Verification {
        id: Option<String>,
        verification: Option<VerificationPropertyValue>,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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
    pub start: Option<DateOrDateTime>,
    pub end: Option<DateOrDateTime>,
    pub time_zone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum DateOrDateTime {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
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
    Array {
        function: RollupFunction,
        array: Vec<PageProperty>,
    },
    Date {
        function: RollupFunction,
        date: Option<DateTime<Utc>>,
    },
    Incomplete {
        function: RollupFunction,
        incomplete: Option<String>,
    },
    Number {
        function: RollupFunction,
        number: Option<Number>,
    },
    Unsupported {
        function: RollupFunction,
        unsupported: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    Average,
    Checked,
    Count,
    CountPerGroup,
    CountValues,
    DateRange,
    EarliestDate,
    Empty,
    LatestDate,
    Max,
    Median,
    Min,
    NotEmpty,
    PercentChecked,
    PercentEmpty,
    PercentNotEmpty,
    PercentPerGroup,
    PercentUnchecked,
    Range,
    ShowOriginal,
    ShowUnique,
    Sum,
    Unchecked,
    Unique,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FilePropertyValue {
    pub name: String,
    #[serde(flatten)]
    pub file: File,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct SelectPropertyValue {
    pub id: Option<String>,
    pub name: Option<String>,
    pub color: Option<Color>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct UniqueIDPropertyValue {
    #[serde(default)]
    #[doc = "This field is null only in extremely rare cases, for instance when it's being used as a Template for a Task in Notion's Project Management Template: https://www.notion.so/templates/notion-projects-and-tasks"]
    pub number: Option<Number>,
    pub prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VerificationPropertyValue {
    pub state: VerificationState,
    pub verified_by: Option<User>,
    pub date: Option<DatePropertyValue>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VerificationState {
    Verified,
    Unverified,
}
