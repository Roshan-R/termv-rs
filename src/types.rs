use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default)]
pub struct Channel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub country: Option<String>,
    pub languages: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub is_nsfw: Option<bool>,
    #[serde(default)]
    pub stream: Stream,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default)]
pub struct Stream {
    #[serde(rename(serialize = "channel", deserialize = "channel"))]
    pub id: Option<String>,
    pub url: Option<String>,
}
