use serde::Deserialize;

pub mod block;
pub mod comment;
pub mod database;
pub mod emoji;
pub mod error;
pub mod file;
pub mod page;
pub mod parent;
pub mod property;
pub mod rich_text;
#[cfg(test)]
mod tests;
pub mod user;

use error::Error;

#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum Response<T> {
    Success(T),
    Error(Error),
}
