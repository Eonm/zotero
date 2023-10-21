use derive_builder::Builder;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Library {
    r#type: String,
    pub id: usize,
    pub name: String,
    pub links: Links,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Links {
    #[serde(alias = "self")]
    pub self_link: Option<Link>,
    pub alternate: Link,
    // Only for collections
    pub up: Option<Link>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Link {
    pub href: String,
    pub r#type: String,
}

/// A struct representing a Zotero collection
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Collection {
    pub key: String,
    pub version: usize,
    pub library: Library,
    pub links: Links,
    pub meta: CollectionMeta,
    pub data: CollectionData,
}

/// This struct can be used to create a new Zotero collection
#[derive(Default, Deserialize, Serialize, Debug, Builder, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct CollectionData {
    #[serde(skip_serializing)]
    pub key: String,
    #[serde(skip_serializing)]
    pub version: usize,
    pub name: String,
    pub parent_collection: StringOrBool,
    pub relations: HashMap<String, String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct CollectionMeta {
    pub num_collections: Option<usize>,
    pub num_items: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntOrBool {
    Bool(bool),
    Int(i64),
}

impl Default for IntOrBool {
    fn default() -> IntOrBool {
        IntOrBool::Bool(false)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrBool {
    Bool(bool),
    String(String),
}

impl Default for StringOrBool {
    fn default() -> StringOrBool {
        StringOrBool::Bool(false)
    }
}
