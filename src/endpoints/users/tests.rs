use crate::{endpoints::users::list::response::ListAllUsersResponse, objects::user::User};

#[test]
fn test_list_200() {
    let result = serde_json::from_str::<ListAllUsersResponse>(include_str!("tests/list_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_retrieve_a_user_200() {
    let result = serde_json::from_str::<User>(include_str!("tests/retrieve_a_user_200.json"));
    assert!(result.is_ok())
}

#[test]
fn test_retrieve_your_tokens_bot_user_200() {
    let result =
        serde_json::from_str::<User>(include_str!("tests/retrieve_your_tokens_bot_user_200.json"));
    assert!(result.is_ok())
}
