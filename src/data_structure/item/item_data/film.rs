use crate::data_structure::shared_fields::Tag;
use crate::data_structure::item::Creator;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use derive_builder::Builder;

///  A film or motion picture. Generally, use this type for artistically-oriented films (including fictional, non-fictional, and documentary films). For other types of video items, use Video Recording.
#[derive(Default, Deserialize, Serialize, Clone, Debug, Builder)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct FilmData {
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
    pub distributor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub date: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub genre: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub video_recording_format: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub running_time: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub language: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub short_title: String,
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
    "film".to_string()
}

use crate::data_structure::ToJson;
impl ToJson for FilmData {}
