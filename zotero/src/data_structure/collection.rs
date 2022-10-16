use derive_builder::Builder;
use std::collections::HashMap;

use serde::de;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

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

///! A struct representing a Zotero collection
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Collection {
    pub key: String,
    pub version: usize,
    pub library: Library,
    pub links: Links,
    pub meta: CollectionMeta,
    pub data: CollectionData,
}

///! This struct can be used to create a new Zotero collection
#[derive(Default, Deserialize, Serialize, Debug, Builder, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct CollectionData {
    #[serde(skip_serializing)]
    pub key: String,
    #[serde(skip_serializing)]
    pub version: usize,
    pub name: String,
    #[serde(deserialize_with = "deserialize_collection_parent")]
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
pub enum IntOrBool {
    r#Bool(bool),
    Int(i64),
}

impl Default for IntOrBool {
    fn default() -> IntOrBool {
        IntOrBool::Bool(false)
    }
}

/// A custom deserializer that deserialize parent_collection value either in bool or in Int.
fn deserialize_meta_numChildren<'de, D>(deserializer: D) -> Result<IntOrBool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Value = serde::Deserialize::deserialize(deserializer)?;
    if s.is_boolean() {
        Ok(IntOrBool::Bool(
            serde_json::from_value::<bool>(s).map_err(de::Error::custom)?,
        ))
    } else if s.is_number() {
        Ok(IntOrBool::Int(
            serde_json::from_value::<i64>(s).map_err(de::Error::custom)?,
        ))
    } else {
        panic!("invalid value")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StringOrBool {
    r#Bool(bool),
    String(String),
}

impl Default for StringOrBool {
    fn default() -> StringOrBool {
        StringOrBool::Bool(false)
    }
}

/// A custom deserializer that deserialize parent_collection value either in bool or in String.
fn deserialize_collection_parent<'de, D>(deserializer: D) -> Result<StringOrBool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Value = serde::Deserialize::deserialize(deserializer)?;
    if s.is_boolean() {
        Ok(StringOrBool::Bool(
            serde_json::from_value::<bool>(s).map_err(de::Error::custom)?,
        ))
    } else if s.is_string() {
        Ok(StringOrBool::String(
            serde_json::from_value::<String>(s).map_err(de::Error::custom)?,
        ))
    } else {
        panic!("invalid value")
    }
}
