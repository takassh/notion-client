use serde::{Deserialize, Serialize};

use super::{
    emoji::{CustomEmoji, Emoji},
    file::{ExternalFile, File},
};

/// Icon used in untagged contexts (like page.rs and block.rs)
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Icon {
    File(File),
    Emoji(Emoji),
    CustomEmoji { custom_emoji: CustomEmoji },
    External { external: ExternalFile },
}

/// Icon with a default value, used in database.rs
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IconWithDefault {
    #[default]
    None,
    File(File),
    Emoji(Emoji),
    CustomEmoji {
        custom_emoji: CustomEmoji,
    },
    External {
        external: ExternalFile,
    },
}
