use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Channel {
    pub name: String,
    pub url: String,
}
