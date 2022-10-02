use serde::{Deserialize, Serialize};

use ::chrono::{DateTime, Utc};
use serde_with::*;
use crate::types::embed::Embed;
use crate::types::user::User;

use super::Snowflake;

// ======= STRUCTS =======

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
/// A Discord message
pub struct Message {
    #[serde_as(as = "DisplayFromStr")]
    /// Message id
    pub id: Snowflake,
    /// The author of the message
    pub author: Option<User>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The channel id for the message
    pub channel_id: Option<Snowflake>,
    /// The content of the message
    pub content: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    /// The timestamp for which the message was created at
    pub timestamp: DateTime<Utc>,
    /// The embeds for the message
    pub embeds: Vec<Embed>,
    /// TTS
    tts: bool,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    /// The timestamp for which the message was edited at
    edited_timestamp: Option<DateTime<Utc>>,
    /// The flags for the message
    flags: u32,
    /// The type of message
    r#type: u8,
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}