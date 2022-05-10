use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'a> {
    pub app: App,
    #[serde(borrow)]
    pub group: BTreeMap<&'a str, Group>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub users: Vec<String>,
    pub files: Vec<String>,
}
