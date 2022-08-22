//! This module contains all Zotero data structures. This data structures can be used for serialization and deserialization.

use serde::{Serialize};

pub mod collection;
pub mod item;
mod shared_fields;

/// Transform a data structure to a json object
pub trait ToJson: Serialize {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
