use crate::item::Creator;
use crate::shared_fields::{ItemCommon, Tag};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use zotero_derive::ItemCommon;

/// A legal case, either published or unpublished.
#[derive(Default, Deserialize, Serialize, Clone, Debug, Builder, ItemCommon)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct CaseData {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub key: String,
    #[builder(setter(skip))]
    pub version: usize,
    #[builder(setter(skip))]
    #[serde(default = "default_document_type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub case_name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub creators: Vec<Creator>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub abstract_note: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub reporter: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub reporter_volume: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub court: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub docket_number: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub first_page: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub history: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub date_decided: String,
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
    "case".to_string()
}

use crate::ToJson;
impl ToJson for CaseData {}
