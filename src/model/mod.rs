use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub app: App,
    pub group: Option<Group>,
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
