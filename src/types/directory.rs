use serde::{Serialize, Deserialize};

use super::acl::AclEntry;

#[derive(Serialize,Deserialize,Clone)]
pub struct Directory {
    pub name: String,
    pub path: String,
    pub acls: Vec<AclEntry>,
}

#[derive(Deserialize)]
pub struct CreateDirectory {
    pub name: String,
}