use serde::{Deserialize, Serialize};

use ::chrono::{DateTime, Utc};
use serde_with::*;

use super::Snowflake;

// ======= STRUCTS =======

#[serde_as]
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// A Discord channel
pub struct Channel {
    #[serde_as(as = "DisplayFromStr")]
    /// Channel id
    pub id: Snowflake,
    // The name of the channel
    pub name: String,
    #[serde(rename="type")]
    /// Channel type
    pub channel_type: String,
    /// Permission bit set
    pub permissions: String
}