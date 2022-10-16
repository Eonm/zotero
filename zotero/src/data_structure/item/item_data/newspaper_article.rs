use crate::data_structure::shared_fields::{Tag, Tagable};
use crate::data_structure::item::Creator;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use derive_builder::Builder;
use zotero_derive::Tagable;

/// An article published in a newspaper (either print or online).
#[derive(Default, Deserialize, Serialize, Clone, Debug, Builder, Tagable)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct NewspaperArticleData {
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
    pub publication_title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub place: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub edition: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub date: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub section: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pages: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub language: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub short_title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub issn: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub url: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub access_date: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub archive: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub archive_location: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub library_catalog: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub call_number: String,
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
    "newspaperArticle".to_string()
}

use crate::data_structure::ToJson;
impl ToJson for NewspaperArticleData {}
