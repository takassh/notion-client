use crate::endpoints::search::title::{
    request::{
        Filter, FilterProperty, FilterValue, SearchByTitleRequest, Sort, SortDirection, Timestamp,
    },
    response::SearchByTitleResponse,
};

#[test]
fn test_search_by_title_request() {
    let request = SearchByTitleRequest {
        query: Some("External tasks".to_string()),
        filter: Some(Filter {
            value: FilterValue::Database,
            property: FilterProperty::Object,
        }),
        sort: Some(Sort {
            direction: SortDirection::Ascending,
            timestamp: Timestamp::LastEditedTime,
        }),
        ..Default::default()
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/search_by_title_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_search_by_title_200() {
    let result = serde_json::from_str::<SearchByTitleResponse>(include_str!(
        "tests/search_by_title_200.json"
    ));
    assert!(result.is_ok())
}

#[test]
fn test_search_by_title_failure() {
    let result = serde_json::from_str::<SearchByTitleResponse>(include_str!(
        "tests/search_by_title_failure_with_missing_unique_id.json"
    ));
    dbg!(&result);
    assert!(result.is_ok())
}
