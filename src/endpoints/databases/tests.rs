use std::collections::{BTreeMap, HashMap};

use serde_json::Number;

use crate::{
    endpoints::databases::{
        create::request::CreateADatabaseRequest,
        query::{
            request::{
                CheckBoxCondition, Filter, NumberCondition, PropertyCondition,
                QueryDatabaseRequest, Sort, SortDirection,
            },
            response::QueryDatabaseResponse,
        },
        update::request::UpdateADatabaseRequest,
    },
    objects::{
        database::{
            Color, Database, DatabaseProperty, Icon, NumberFormat, NumberPropertyValue,
            OptionValue, RelationPropertyValue, RollupFunction, RollupPropertyValue,
            SelectPropertyValue,
        },
        emoji::Emoji,
        file::ExternalFile,
        parent::Parent,
        rich_text::{RichText, Text},
    },
};

use super::query::request::FilterType;

#[test]
fn test_create_request() {
    let mut properties = BTreeMap::new();
    properties.insert(
        "Name".to_string(),
        DatabaseProperty::Title {
            id: None,
            name: None,
            title: HashMap::new(),
        },
    );
    properties.insert(
        "Description".to_string(),
        DatabaseProperty::RichText {
            id: None,
            name: None,
            rich_text: HashMap::new(),
        },
    );
    properties.insert(
        "In stock".to_string(),
        DatabaseProperty::Checkbox {
            id: None,
            name: None,
            checkbox: HashMap::new(),
        },
    );
    properties.insert(
        "Food group".to_string(),
        DatabaseProperty::Select {
            id: None,
            name: None,
            select: SelectPropertyValue {
                options: vec![
                    OptionValue {
                        name: "🥦Vegetable".to_string(),
                        color: Some(Color::Green),
                        id: None,
                    },
                    OptionValue {
                        name: "🍎Fruit".to_string(),
                        color: Some(Color::Red),
                        id: None,
                    },
                    OptionValue {
                        name: "💪Protein".to_string(),
                        color: Some(Color::Yellow),
                        id: None,
                    },
                ],
            },
        },
    );
    properties.insert(
        "Price".to_string(),
        DatabaseProperty::Number {
            id: None,
            name: None,
            number: NumberPropertyValue {
                format: NumberFormat::Dollar,
            },
        },
    );
    properties.insert(
        "Last ordered".to_string(),
        DatabaseProperty::Date {
            id: None,
            name: None,
            date: HashMap::new(),
        },
    );
    properties.insert(
        "Meals".to_string(),
        DatabaseProperty::Relation {
            id: None,
            name: None,
            relation: RelationPropertyValue {
                database_id: Some("668d797c-76fa-4934-9b05-ad288df2d136".to_string()),
                ..Default::default()
            },
        },
    );
    properties.insert(
        "Number of meals".to_string(),
        DatabaseProperty::Rollup {
            id: None,
            name: None,
            rollup: RollupPropertyValue {
                rollup_property_name: "Name".to_string(),
                relation_property_name: "Meals".to_string(),
                function: RollupFunction::Count,
                ..Default::default()
            },
        },
    );
    properties.insert(
        "Store availability".to_string(),
        DatabaseProperty::MultiSelect {
            id: None,
            name: None,
            multi_select: SelectPropertyValue {
                options: vec![
                    OptionValue {
                        name: "Duc Loi Market".to_string(),
                        color: Some(Color::Blue),
                        ..Default::default()
                    },
                    OptionValue {
                        name: "Rainbow Grocery".to_string(),
                        color: Some(Color::Gray),
                        ..Default::default()
                    },
                    OptionValue {
                        name: "Nijiya Market".to_string(),
                        color: Some(Color::Purple),
                        ..Default::default()
                    },
                    OptionValue {
                        name: "Gus's Community Market".to_string(),
                        color: Some(Color::Yellow),
                        ..Default::default()
                    },
                ],
            },
        },
    );
    properties.insert(
        "+1".to_string(),
        DatabaseProperty::People {
            id: None,
            name: None,
            people: HashMap::new(),
        },
    );
    properties.insert(
        "Photo".to_string(),
        DatabaseProperty::Files {
            id: None,
            name: None,
            files: HashMap::new(),
        },
    );

    let request = CreateADatabaseRequest {
        parent: Parent::PageId {
            page_id: "98ad959b-2b6a-4774-80ee-00246fb0ea9b".to_string(),
        },
        icon: Some(Icon::Emoji(Emoji::Emoji {
            emoji: "📝".to_string(),
        })),
        cover: Some(crate::objects::file::File::External {
            external: ExternalFile {
                url: "https://website.domain/images/image.png".to_string(),
            },
        }),
        title: Some(vec![RichText::Text {
            text: Text {
                content: "Grocery List".to_string(),
                link: None,
            },
            annotations: None,
            plain_text: None,
            href: None,
        }]),
        properties,
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/create_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_query_request() {
    let request = QueryDatabaseRequest {
        filter: Some(Filter::Or {
            or: vec![
                FilterType::Property {
                    property: "In stock".to_string(),
                    condition: PropertyCondition::Checkbox(CheckBoxCondition::Equals(true)),
                },
                FilterType::Property {
                    property: "Cost of next trip".to_string(),
                    condition: PropertyCondition::Number(NumberCondition::GreaterThanOrEqualTo(
                        Number::from(2),
                    )),
                },
            ],
        }),
        sorts: Some(vec![Sort::Property {
            property: "Last ordered".to_string(),
            direction: SortDirection::Ascending,
        }]),
        ..Default::default()
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/query_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_update_request() {
    let mut properties = BTreeMap::new();

    properties.insert(
        "Store availability".to_string(),
        Some(DatabaseProperty::MultiSelect {
            id: None,
            name: None,
            multi_select: SelectPropertyValue {
                options: vec![
                    OptionValue {
                        name: "Duc Loi Market".to_string(),

                        ..Default::default()
                    },
                    OptionValue {
                        name: "Rainbow Grocery".to_string(),

                        ..Default::default()
                    },
                    OptionValue {
                        name: "Gus's Community Market".to_string(),

                        ..Default::default()
                    },
                    OptionValue {
                        name: "The Good Life Grocery".to_string(),
                        color: Some(Color::Orange),
                        ..Default::default()
                    },
                ],
            },
        }),
    );
    properties.insert("+1".to_string(), None);
    properties.insert(
        "Photo".to_string(),
        Some(DatabaseProperty::Files {
            id: None,
            name: None,
            files: HashMap::new(),
        }),
    );

    let request = UpdateADatabaseRequest {
        title: Some(vec![RichText::Text {
            text: Text {
                content: "Today's grocery list".to_string(),
                link: None,
            },
            annotations: None,
            plain_text: None,
            href: None,
        }]),
        description: Some(vec![RichText::Text {
            text: Text {
                content: "Grocery list for just kale 🥬".to_string(),
                link: None,
            },
            annotations: None,
            plain_text: None,
            href: None,
        }]),
        properties,
    };

    let result = serde_json::to_string_pretty(&request).unwrap();
    let actual = include_str!("tests/update_request.json");
    assert_eq!(result, actual.to_string())
}

#[test]
fn test_create_200() {
    let result = serde_json::from_str::<Database>(include_str!("tests/create_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_query_200() {
    let result =
        serde_json::from_str::<QueryDatabaseResponse>(include_str!("tests/query_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_retrieve_200() {
    let result = serde_json::from_str::<Database>(include_str!("tests/retrieve_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_update_200() {
    let result = serde_json::from_str::<Database>(include_str!("tests/update_200.json"));
    assert!(result.is_ok())
}
