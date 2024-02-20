use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct QueryDatabaseRequest {
    pub filter: Option<Filter>,
    pub sorts: Option<Vec<Sort>>,
    pub start_cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged, rename_all = "snake_case")]
pub enum Filter {
    Value {
        #[serde(flatten)]
        filter_type: FilterType,
    },
    And {
        and: Vec<FilterType>,
    },
    Or {
        or: Vec<FilterType>,
    },
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged, rename_all = "snake_case")]
pub enum FilterType {
    Property {
        property: String,
        #[serde(flatten)]
        condition: PropertyCondition,
    },
    Timestamp {
        timestamp: Timestamp,
        #[serde(flatten)]
        condition: TimestampCondition,
    },
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Timestamp {
    CreatedTime,
    LastEditedTime,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged, rename_all = "snake_case")]
pub enum Sort {
    Property {
        property: String,
        direction: SortDirection,
    },
    Timestamp {
        timestamp: Timestamp,
        direction: SortDirection,
    },
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

use chrono::{DateTime, Utc};
use serde::{ser::SerializeMap, Serializer};
use serde_json::Number;

fn serialize_to_true<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bool(true)
}

fn serialize_to_empty_object<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_map(Some(0))?.end()
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PropertyCondition {
    Checkbox(CheckBoxCondition),
    Date(DateCondition),
    Files(FilesCondition),
    Formula(FormulaCondition),
    MultiSelect(MultiSelectCondition),
    Number(NumberCondition),
    People(PeopleCondition),
    Relation(RelationCondition),
    RichText(RichTextCondition),
    Rollup(Box<RollupCondition>),
    Select(SelectCondition),
    Status(StatusCondition),
    Timestamp(TimestampCondition),
    ID(IDCondition),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CheckBoxCondition {
    Equals(bool),
    DoesNotEqual(bool),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DateCondition {
    After(DateTime<Utc>),
    Before(DateTime<Utc>),
    Equals(DateTime<Utc>),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
    #[serde(serialize_with = "serialize_to_empty_object")]
    NextMonth,
    #[serde(serialize_with = "serialize_to_empty_object")]
    NextWeek,
    #[serde(serialize_with = "serialize_to_empty_object")]
    NextYear,
    OnOrAfter(DateTime<Utc>),
    OnOrBefore(DateTime<Utc>),
    #[serde(serialize_with = "serialize_to_empty_object")]
    PastMonth,
    #[serde(serialize_with = "serialize_to_empty_object")]
    PastWeek,
    #[serde(serialize_with = "serialize_to_empty_object")]
    PastYear,
    #[serde(serialize_with = "serialize_to_empty_object")]
    ThisWeek,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FilesCondition {
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FormulaCondition {
    Checkbox(CheckBoxCondition),
    Date(DateCondition),
    Number(NumberCondition),
    String(RichTextCondition),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MultiSelectCondition {
    Contains(String),
    DoesNotContain(String),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NumberCondition {
    DoesNotEqual(Number),
    Equals(Number),
    GreaterThan(Number),
    GreaterThanOrEqualTo(Number),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
    LessThanOrEqualTo(Number),
    LessThan(Number),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PeopleCondition {
    Contains(String),
    DoesNotContain(String),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RelationCondition {
    Contains(String),
    DoesNotContain(String),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RichTextCondition {
    Contains(String),
    DoesNotContain(String),
    DoesNotEqual(String),
    EndsWith(String),
    Equals(String),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
    StartsWith(String),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RollupCondition {
    Any(PropertyCondition),
    Every(PropertyCondition),
    None(PropertyCondition),
    Date(DateCondition),
    Number(NumberCondition),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SelectCondition {
    Equals(String),
    DoesNotEqual(String),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StatusCondition {
    Equals(String),
    DoesNotEqual(String),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TimestampCondition {
    CreatedTime(DateCondition),
    LastEditedTime(DateCondition),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IDCondition {
    DoesNotEqual(Number),
    Equals(Number),
    GreaterThan(Number),
    GreaterThanOrEqualTo(Number),
    LessThanOrEqualTo(Number),
    LessThan(Number),
}
