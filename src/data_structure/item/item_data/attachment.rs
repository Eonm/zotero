use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

/// A standalone attachment file (e.g., a PDF, JPEG, DOCX, PPTX, XLSX, or ODT file). Standalone attachment files have limited functionality in Zotero (e.g., they cannot be properly searched or cited). Always attach files to proper Zotero items.
#[derive(Default, Deserialize, Serialize, Clone, Debug, Builder)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct AttachmentData {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub access_date: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub charset: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub content_type: String,
    #[serde(skip_serializing)]
    pub date_added: String,
    pub date_modified: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub filename: String,
    pub item_type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub key: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub link_mode: String,
    pub md5: Option<String>,
    pub mtime: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub note: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub parent_item: String,
    pub relations: HashMap<String, String>,
    pub collection: Vec<String>,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub url: String,
    pub version: usize,
}

use crate::data_structure::ToJson;
impl ToJson for AttachmentData {}
