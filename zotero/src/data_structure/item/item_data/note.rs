use crate::data_structure::shared_fields::{Tag, ItemCommon};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use derive_builder::Builder;
use zotero_derive::ItemCommon;

fn note_title() -> String {
    "Note".to_string()
}

/// A standalone note. Notes can be used for organizing and annotating in Zotero. If you cite a standalone note, Zotero will use the first 120 characters as the item title (and will treat the note as an author-less and date-less item). Citing notes is not a reliable way to add standalone commentary to a bibliography or reference list.
#[derive(Default, Deserialize, Serialize, Clone, Debug, Builder, ItemCommon)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct NoteData {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub key: String,
    #[builder(setter(skip))]
    pub version: usize,
    #[builder(setter(skip))]
    #[serde(skip_serializing, default="note_title")]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub parent_item: String,
    #[builder(setter(skip))]
    #[serde(default = "default_document_type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub note: String,
    pub tags: Vec<Tag>,
    pub relations: HashMap<String, String>,
    #[serde(skip_serializing)]
    pub date_added: String,
    #[serde(skip_serializing)]
    pub date_modified: String,
}

fn default_document_type() -> String {
    "note".to_string()
}

use crate::data_structure::ToJson;
impl ToJson for NoteData {}
