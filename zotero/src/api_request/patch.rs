use std::error;

use serde::Serialize;
use serde_json::value::Value;

/// Perform patch operations on Zotero items and collections.
/// ```no_run
/// use zotero::ZoteroInit;
/// use zotero::Patch;
/// use zotero::Get;
/// use zotero::data_structure::item::ItemType;
///
/// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
/// let remote_item = z.get_item("B8ZNE3GH", None);
///
/// if let Ok(mut result) = remote_item {
///     if let ItemType::Book(bookdata) = &mut result.data {
///         bookdata.title = "A new title".to_string();
///         bookdata.publisher = "Doe editions".to_string();
///         bookdata.creators = vec!();
///         z.update_item(&bookdata.key, &bookdata);
///     };
/// };
///```
pub trait Patch<'a> {
    fn patch_request<T: Serialize>(
        &self,
        params: &str,
        json_body: T,
    ) -> Result<Value, Box<dyn error::Error>>;
    fn get_id(&self) -> &'a str;

    /// Update a zotero item.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Patch;
    /// # use zotero::Get;
    /// # use zotero::data_structure::item::ItemType;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let remote_item = z.get_item("B8ZNE3GH", None);
    ///
    /// if let Ok(mut result) = remote_item {
    ///     if let ItemType::Book(bookdata) = &mut result.data {
    ///         bookdata.title = "A new title".to_string();
    ///         bookdata.publisher = "Doe editions".to_string();
    ///         bookdata.creators = vec!();
    ///         z.update_item(&bookdata.key, &bookdata);
    ///     };
    /// };
    ///```
    fn update_item<T: Serialize, S: AsRef<str> + std::fmt::Display>(
        &self,
        item_key: S,
        item_data: T,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!("/items/{}", item_key);
        self.patch_request(&params, &item_data)
    }
}
