use serde::{Deserialize, Serialize};
// #[derive(Serialize, Deserialize, Debug)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Channel {
    pub name: String,
    pub url: String,
    pub categories: Vec<Categories>,
    pub countries: Vec<Countries>,
    pub languages: Vec<Languages>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Categories {
    pub name: String,
    slug: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Countries {
    pub name: String,
    code: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Languages {
    pub name: String,
    code: String,
}