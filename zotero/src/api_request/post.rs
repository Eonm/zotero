use std::error;

use serde::Serialize;
use serde_json::value::Value;

/// Perform post operations on Zotero items and collections.
/// ```no_run
/// use zotero::ZoteroInit;
/// use zotero::Post;
/// use zotero::data_structure::item::{BookData, BookDataBuilder, Creator, CreatorBuilder};
///
/// let creators : Vec<Creator> = vec![
///     CreatorBuilder::default()
///         .creator_type("author")
///         .first_name("John")
///         .last_name("Doe")
///         .build()
///         .unwrap()
/// ];
///
/// let new_book : BookData = BookDataBuilder::default()
///     .title("A title")
///     .creators(creators)
///     .item_type("book")
///     .build()
///     .unwrap();
///
/// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
/// let new_item = z.create_new_item(new_book);
/// ```
pub trait Post<'a> {
    fn post_request<T: Serialize, S: AsRef<str> + std::fmt::Display>(
        &self,
        params: S,
        json_body: T,
    ) -> Result<Value, Box<dyn error::Error>>;
    fn get_id(&self) -> &'a str;

    fn create_new_item<T: Serialize>(&self, item: T) -> Result<Value, Box<dyn error::Error>> {
        let params = "/items".to_string();
        self.post_request(params, vec![&item])
    }

    /// Create multiple items
    fn create_new_items<T: Serialize>(&self, item: Vec<T>) -> Result<Value, Box<dyn error::Error>> {
        let params = "/items".to_string();
        self.post_request(params, &item)
    }

    /// Create new collection
    fn create_new_collection<T: Serialize>(&self, item: T) -> Result<Value, Box<dyn error::Error>> {
        let params = "/collections".to_string();
        self.post_request(params, vec![&item])
    }

    /// Create new collections
    fn create_new_collections<T: Serialize>(
        &self,
        item: Vec<T>,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = "/collections".to_string();
        self.post_request(params, &item)
    }
}
