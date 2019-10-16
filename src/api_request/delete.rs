use serde_json::value::Value;
use std::error;

/// Perform delete operations on Zotero items and collections.
pub trait Delete<'a> {
    fn delete_request(
        &self,
        params: &str,
        last_version: &str,
    ) -> Result<Value, Box<dyn error::Error>>;
    fn get_id(&self) -> &'a str;

    /// Delete a Zotero item.
    fn delete_item<S: AsRef<str> + std::fmt::Display>(
        &self,
        item_key: S,
        last_version: S,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!("items/{}", item_key);
        self.delete_request(&params, last_version.as_ref())
    }

    /// Delete a Zotero collection.
    fn delete_collection<S: AsRef<str> + std::fmt::Display>(
        &self,
        item_key: S,
        last_version: S,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!("collection/{}", item_key);
        self.delete_request(&params, last_version.as_ref())
    }

    /// Delete a Zotero search.
    fn delete_search<S: AsRef<str> + std::fmt::Display>(
        &self,
        search_key: S,
        last_version: S,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!("searches?searchKey={}", search_key);
        self.delete_request(&params, last_version.as_ref())
    }

    /// Delete a Zotero tag.
    fn delete_tag<S: AsRef<str> + std::fmt::Display>(
        &self,
        tag_key: S,
        last_version: S,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!("tags?tag={}", tag_key);
        self.delete_request(&params, last_version.as_ref())
    }
}
