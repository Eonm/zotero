use crate::data_structure::item::Creator;
use derive_builder::Builder;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// A book or similar published item. For government documents, technical reports, manuals, etc., use Report instead. This item type can also be adapted to fit many types of unusual items.
#[derive(Default, Deserialize, Serialize, Clone, Debug, Builder)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct BookData {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub key: String,
    #[serde(skip)]
    #[builder(setter(skip))]
    pub version: usize,
    // #[builder(setter(skip))]
    #[serde(default = "default_document_type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub title: String,
    // #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub creators: Vec<Creator>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub abstract_note: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub series: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub series_number: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub volume: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub number_of_volumes: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub edition: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub place: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub publisher: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub date: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub num_pages: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub language: String,
    #[serde(skip_serializing_if = "String::is_empty", default, rename = "ISBN")]
    pub isbn: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub short_title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub url: String,
    #[serde(skip_serializing)]
    pub access_date: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub archive: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub archive_location: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub library_catalog: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub call_number: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub rights: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub extra: String,
    pub tags: Vec<String>,
    pub collections: Vec<String>,
    pub relations: HashMap<String, String>,
    #[serde(skip_serializing)]
    pub date_added: String,
    #[serde(skip_serializing)]
    pub date_modified: String,
}

fn default_document_type() -> String {
    "book".to_string()
}

use crate::data_structure::ToJson;
impl ToJson for BookData {}
