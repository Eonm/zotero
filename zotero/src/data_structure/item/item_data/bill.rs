use crate::data_structure::item::Creator;
use crate::data_structure::shared_fields::{ItemCommon, Tag};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use zotero_derive::ItemCommon;

/// A proposed piece of legislation.
#[derive(Default, Deserialize, Serialize, Clone, Debug, Builder, ItemCommon)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct BillData {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub key: String,
    #[builder(setter(skip))]
    pub version: usize,
    #[builder(setter(skip))]
    #[serde(default = "default_document_type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub title: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub creators: Vec<Creator>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub abstract_note: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub bill_number: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub code: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub code_volume: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub section: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub code_pages: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub legislative_body: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub session: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub history: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub date: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub language: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub url: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub access_date: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub short_title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub rights: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub extra: String,
    pub tags: Vec<Tag>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub collections: Option<Vec<String>>,
    pub relations: HashMap<String, String>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub date_added: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub date_modified: String,
}

fn default_document_type() -> String {
    "bill".to_string()
}

use crate::data_structure::ToJson;
impl ToJson for BillData {}
