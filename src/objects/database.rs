use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{emoji::Emoji, file::File, parent::Parent, rich_text::RichText, user::User};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Database {
    pub id: Option<String>,
    pub created_time: DateTime<Utc>,
    pub created_by: Option<User>,
    pub last_edited_time: DateTime<Utc>,
    pub last_edited_by: Option<User>,
    pub title: Vec<RichText>,
    pub description: Vec<RichText>,
    pub icon: Option<Icon>,
    pub cover: Option<File>,
    pub properties: HashMap<String, DatabaseProperty>,
    pub parent: Parent,
    pub url: String,
    pub archived: bool,
    pub is_inline: bool,
    pub public_url: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Icon {
    #[default]
    None,
    File(File),
    Emoji(Emoji),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DatabaseProperty {
    Checkbox {
        id: Option<String>,
        name: Option<String>,
        checkbox: HashMap<(), ()>,
    },
    CreatedBy {
        id: Option<String>,
        name: Option<String>,
        created_by: HashMap<(), ()>,
    },
    CreatedTime {
        id: Option<String>,
        name: Option<String>,
        created_time: HashMap<(), ()>,
    },
    Date {
        id: Option<String>,
        name: Option<String>,
        date: HashMap<(), ()>,
    },
    Email {
        id: Option<String>,
        name: Option<String>,
        email: HashMap<(), ()>,
    },
    Files {
        id: Option<String>,
        name: Option<String>,
        files: HashMap<(), ()>,
    },
    Formula {
        id: Option<String>,
        name: Option<String>,
        formula: FormulaPropertyValue,
    },
    LastEditedBy {
        id: Option<String>,
        name: Option<String>,
        last_edited_by: HashMap<(), ()>,
    },
    LastEditedTime {
        id: Option<String>,
        name: Option<String>,
        last_edited_time: HashMap<(), ()>,
    },
    MultiSelect {
        id: Option<String>,
        name: Option<String>,
        multi_select: SelectPropertyValue,
    },
    Number {
        id: Option<String>,
        name: Option<String>,
        number: NumberPropertyValue,
    },
    People {
        id: Option<String>,
        name: Option<String>,
        people: HashMap<(), ()>,
    },
    PhoneNumber {
        id: Option<String>,
        name: Option<String>,
        phone_number: HashMap<(), ()>,
    },
    Relation {
        id: Option<String>,
        name: Option<String>,
        relation: RelationPropertyValue,
    },
    RichText {
        id: Option<String>,
        name: Option<String>,
        rich_text: HashMap<(), ()>,
    },
    Rollup {
        id: Option<String>,
        name: Option<String>,
        rollup: RollupPropertyValue,
    },
    Select {
        id: Option<String>,
        name: Option<String>,
        select: SelectPropertyValue,
    },
    Status {
        id: Option<String>,
        name: Option<String>,
        status: StatusPropertyValue,
    },
    Title {
        id: Option<String>,
        name: Option<String>,
        title: HashMap<(), ()>,
    },
    Url {
        id: Option<String>,
        name: Option<String>,
        url: HashMap<(), ()>,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    #[default]
    Blue,
    Brown,
    Default,
    Gray,
    Green,
    Orange,
    Pink,
    Purple,
    Red,
    Yellow,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FormulaPropertyValue {
    pub expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct NumberPropertyValue {
    pub format: NumberFormat,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NumberFormat {
    ArgentinePeso,
    Baht,
    AustralianDollar,
    CanadianDollar,
    ChileanPeso,
    ColombianPeso,
    DanishKrone,
    Dirham,
    Dollar,
    Euro,
    Forint,
    Franc,
    HongKongDollar,
    Koruna,
    Krona,
    Leu,
    Lira,
    MexicanPeso,
    NewTaiwanDollar,
    NewZealandDollar,
    NorwegianKrone,
    Number,
    NumberWithCommas,
    Percent,
    PhilippinePeso,
    Pound,
    PeruvianSol,
    Rand,
    Real,
    Ringgit,
    Riyal,
    Ruble,
    Rupee,
    Rupiah,
    Shekel,
    SingaporeDollar,
    UruguayanPeso,
    Yen,
    Yuan,
    Won,
    Zloty,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct RelationPropertyValue {
    pub database_id: Option<String>,
    pub synced_property_id: Option<String>,
    pub synced_property_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct RollupPropertyValue {
    pub rollup_property_name: String,
    pub relation_property_name: String,
    pub function: RollupFunction,
    pub relation_property_id: Option<String>,
    pub rollup_property_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    #[default]
    Average,
    Checked,
    CountPerGroup,
    Count,
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
    Unchecked,
    Unique,
    ShowOriginal,
    ShowUnique,
    Sum,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct StatusPropertyValue {
    pub options: Vec<SelectPropertyValue>,
    pub groups: Vec<Group>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Group {
    pub color: Color,
    pub id: Option<String>,
    pub name: String,
    pub option_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SelectPropertyValue {
    pub options: Vec<OptionValue>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct OptionValue {
    pub name: String,
    pub color: Option<Color>,
    pub id: Option<String>,
}
