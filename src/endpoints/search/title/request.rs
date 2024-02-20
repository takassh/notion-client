use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct SearchByTitleRequest {
    pub query: Option<String>,
    pub filter: Option<Filter>,
    pub sort: Option<Sort>,
    pub start_cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Sort {
    pub direction: SortDirection,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Timestamp {
    LastEditedTime,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Filter {
    pub value: FilterValue,
    pub property: FilterProperty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FilterValue {
    Page,
    Database,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FilterProperty {
    Object,
}
