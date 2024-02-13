use serde::{Deserialize, Serialize};

use crate::objects::user::User;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ListAllUsersResponse {
    pub results: Vec<User>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}
