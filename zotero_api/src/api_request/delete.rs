use serde_json::value::Value;
use std::error;

extern crate url;
use url::form_urlencoded::byte_serialize;

/// Perform delete operations on Zotero items and collections.
pub trait Delete<'a> {
    fn delete_request(
        &self,
        params: &str,
        last_version: &str,
    ) -> Result<Value, Box<dyn error::Error>>;
    fn get_id(&self) -> &'a str;

    /// Delete a Zotero item.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// z.delete_item("PJTUB2WE", "2050");
    /// ```
    fn delete_item<S: AsRef<str> + std::fmt::Display>(
        &self,
        item_key: S,
        last_version: S,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!("items/{}", item_key);
        self.delete_request(&params, last_version.as_ref())
    }

    /// Delete a Zotero item.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// z.delete_items(vec!["PJTUB2WE", "YXT5PJU9"], "2050");
    /// ```
    fn delete_items<S: AsRef<str> + std::fmt::Display>(
        &self,
        items_keys: Vec<S>,
        last_version: S,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!(
            "items/{}",
            items_keys
                .iter()
                .map(|elem| elem.as_ref())
                .collect::<Vec<&str>>()
                .join(" || ")
        );
        self.delete_request(&params, last_version.as_ref())
    }

    /// Delete a Zotero collection.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// z.delete_collection("TYQDGEZR", "2050");
    /// ```
    fn delete_collection<S: AsRef<str> + std::fmt::Display>(
        &self,
        item_key: S,
        last_version: S,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!("collections/{}", item_key);
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
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// z.delete_tag("history", "2050");
    /// ```
    fn delete_tag<S: AsRef<str> + std::fmt::Display>(
        &self,
        tag_key: S,
        last_version: S,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!("tags?tag={}", tag_key);
        self.delete_request(&params, last_version.as_ref())
    }

    /// Delete multiple Zotero tags.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// z.delete_tags(vec!["history", "philosophy", "art"], "2050");
    /// ```
    fn delete_tags<S: AsRef<str> + std::fmt::Display>(
        &self,
        tags_keys: Vec<S>,
        last_version: S,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!(
            "tags?tag={}",
            tags_keys
                .iter()
                .map(|elem| byte_serialize(elem.as_ref().as_bytes()).collect())
                .collect::<Vec<String>>()
                .join(" || ")
        );
        self.delete_request(&params, last_version.as_ref())
    }
}
