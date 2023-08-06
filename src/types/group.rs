use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Group {
    pub cn: String,
    pub description: String,
    pub member: Option<Vec<String>>, // "uid=thbellem,ou=people,dc=uca,dc=fr"
    pub owner: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateGroup {
    pub cn: String,
    pub description: String,
    pub owner: String,
}