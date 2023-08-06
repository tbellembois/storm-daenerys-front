use std::fmt;

use posix_acl::ACLEntry;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Qualifier {
    /// Unrecognized/corrupt entries
    Undefined,
    /// Permissions for owner of the file
    UserObj,
    /// Permissions for owning group of the file
    GroupObj,
    /// Permissions for everyone else not covered by the ACL
    Other,
    /// Permissions for user with UID `u32` value
    User(u32),
    /// Permissions for group with GID `u32` value
    Group(u32),
    /// Auto-generated entry
    Mask,
}

impl fmt::Debug for Qualifier {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            Qualifier::Undefined => write!(f, "Undefined"),
            Qualifier::UserObj => write!(f, "UserObj"),
            Qualifier::GroupObj => write!(f, "GroupObj"),
            Qualifier::Other => write!(f, "Other"),
            Qualifier::User(u) => write!(f, "User:{}", u),
            Qualifier::Group(g) => write!(f, "Group:{}", g),
            Qualifier::Mask => write!(f, "Mask"),
        }
        
    }

}

#[derive(Serialize, Deserialize,Clone)]
pub struct AclEntry {
    pub qualifier: Qualifier, // the subject of a permission grant
    pub perm: u32,
}

impl fmt::Debug for AclEntry {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.qualifier, self.perm)
    }

}

impl AclEntry {
    
    pub fn from_posix_acl_entry(entry: &ACLEntry) -> Self {

        let qualifier = match entry.qual {
            posix_acl::Qualifier::Undefined => Qualifier::Undefined,
            posix_acl::Qualifier::UserObj => Qualifier::UserObj,
            posix_acl::Qualifier::GroupObj => Qualifier::GroupObj,
            posix_acl::Qualifier::Other => Qualifier::Other,
            posix_acl::Qualifier::User(u) => Qualifier::User(u),
            posix_acl::Qualifier::Group(g) => Qualifier::Group(g),
            posix_acl::Qualifier::Mask => Qualifier::Mask,
        };

        let perm = entry.perm;

        AclEntry { qualifier, perm }

    }

    pub fn to_posix_acl_entry(&self) -> ACLEntry {

        let qual = match self.qualifier {
            Qualifier::Undefined => posix_acl::Qualifier::Undefined,
            Qualifier::UserObj => posix_acl::Qualifier::UserObj,
            Qualifier::GroupObj => posix_acl::Qualifier::GroupObj,
            Qualifier::Other => posix_acl::Qualifier::Other,
            Qualifier::User(u) => posix_acl::Qualifier::User(u),
            Qualifier::Group(g) => posix_acl::Qualifier::Group(g),
            Qualifier::Mask => posix_acl::Qualifier::Mask,
        };

        let perm = self.perm;

        posix_acl::ACLEntry{ qual, perm }

    } 

}

#[derive(Deserialize)]
pub struct SetAcl {
    pub name: String,
    pub acls: Vec<AclEntry>,
}
