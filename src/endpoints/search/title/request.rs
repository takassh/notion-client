use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct SearchByTitleRequest {
    pub query: Option<String>,
    pub sort: Option<Sort>,
    pub filter: Option<Filter>,
    pub start_cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Sort {
    pub direction: SortDirection,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
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
    value: FilterValue,
    property: FilterProperty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
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
