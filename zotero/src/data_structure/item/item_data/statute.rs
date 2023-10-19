use crate::data_structure::item::Creator;
use crate::data_structure::shared_fields::{ItemCommon, Tag};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use derive_builder::Builder;
use zotero_derive::ItemCommon;

/// A law or other piece of enacted legislation.
#[derive(Default, Deserialize, Serialize, Clone, Debug, Builder, ItemCommon)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct StatuteData {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub key: String,
    #[builder(setter(skip))]
    pub version: usize,
    #[builder(setter(skip))]
    #[serde(default = "default_document_type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub name_of_act: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub creators: Vec<Creator>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub abstract_note: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub code: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub code_number: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub public_law_number: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub date_enacted: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pages: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub section: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub session: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub history: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub language: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        default,
        rename = "short_title"
    )]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub url: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub access_date: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub rights: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub extra: String,
    pub tags: Vec<Tag>,
    pub collections: Option<Vec<String>>,
    pub relations: HashMap<String, String>,
    #[serde(skip_serializing)]
    pub date_added: String,
    #[serde(skip_serializing)]
    pub date_modified: String,
}

fn default_document_type() -> String {
    "statute".to_string()
}

use crate::data_structure::ToJson;
impl ToJson for StatuteData {}
