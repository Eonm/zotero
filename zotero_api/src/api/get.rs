use http::header::AUTHORIZATION;
use serde_json::value::Value;
use std::error;
use std::fmt::format;
use zotero_data::collection::Collection;
use zotero_data::item::Item;

use http::Request;
use http::request::Builder;

/// Perform get operations on Zotero items and collections.
/// ```no_run
/// use zotero_api::ZoteroInit;
/// use zotero_api::Get;
///
/// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
/// let items = z.get_items(None);
/// ```
pub trait Get<'a> {
    fn get_base_url(&self) -> String;
    fn get_id(&self) -> &'a str;
    fn get_api_key(&self) -> &'a str;

    fn get_request<I: Into<Option<&'a str>>>(&self, params: String, extra_params: I) -> Request<&'static str> {
        let extra_params: Option<&str> = extra_params.into();
        Request::builder()
            .method("GET")
            .uri(match extra_params {
                None => format!("{}{}", self.get_base_url(), params),
                Some(extra_params) => format!("{}{}?{}", self.get_base_url(), params, extra_params),
            })
            .header(AUTHORIZATION, format!("Bearer {}", self.get_api_key()))
            .body("").unwrap()
    }

    /// Retreive information about an api key and it's privileges.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection = z.get_api_key_info("bZARysJ579K5SdmYuaAJ");
    /// ```
    fn get_api_key_info<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<&'static str>  {
        let params = format!("/keys/{}", self.get_api_key());
        self.get_request(params, extra_params)
    }

    /// Retreive specific item in the library by it's ID.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let items = z.get_item("B8ZNE3GH", None);
    /// ```
    fn get_item<I: Into<Option<&'a str>>>(
        &self,
        item_id: &'a str,
        extra_params: I,
    ) -> Request<&'static str> {
        let params: String = format!("/items/{}", item_id);
        self.get_request(params, extra_params)
    }

    /// Retreive all items in the library, excluding trashed items.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let items = z.get_items(None);
    /// ```
    fn get_items<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<&'static str> {
        let params = "/items".to_string();
        self.get_request(params, extra_params)
    }

    /// Retreive all child items of a specific item
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let items = z.get_child_items("B8ZNE3GH", None);
    /// ```
    fn get_child_items<I: Into<Option<&'a str>>>(
        &self,
        item_id: &'a str,
        extra_params: I,
    ) -> Request<&'static str> {
        let params: String = format!("/items/{}/children", item_id);
        self.get_request(params, extra_params)
    }

    /// Retreive top-level items in the library, excluding trashed items.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let top_items = z.get_top_items(None);
    /// ```
    fn get_top_items<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<&'static str> {
        let params = "/items/top".to_string();
        self.get_request(params, extra_params)
    }

    /// Retreive items in the trash.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let trashed_items = z.get_trashed_items(None);
    /// ```
    fn get_trashed_items<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<&'static str> {
        let params = "/items/trash".to_string();
        self.get_request(params, extra_params)
    }

    /// Retreive items in "My publications".
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let my_publications = z.get_publications(None);
    /// ```
    fn get_publications<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<&'static str> {
        let params = "/publications/items".to_string();
        self.get_request(params, extra_params)
    }

    /// Retreive a collection by it's id.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection = z.get_collection("AYVWED", None);
    /// ```
    fn get_collection<I: Into<Option<&'a str>>>(
        &self,
        collection_id: &'a str,
        extra_params: I,
    ) -> Request<&'static str> {
        let params = format!("/collections/{}", collection_id);
        self.get_request(params, extra_params)
    }


    /// Retreive all collections.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collections = z.get_collections(None);
    /// ```
    fn get_collections<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<&'static str> {
        let params = "/collections".to_string();
        self.get_request(params, extra_params)
    }

    /// Retreive top level collections.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let top_collections = z.get_top_collections(None);
    /// ```
    fn get_top_collections<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<&'static str> {
        let params = "/collections/top".to_string();
        self.get_request(params, extra_params)
    }

    /// Retreive all items for a given collection
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection_items = z.get_collection_items("AYVWED", None);
    /// ```
    fn get_collection_items<I: Into<Option<&'a str>>>(
        &self,
        collection_id: &'a str,
        extra_params: I,
    ) -> Request<&'static str> {
        let params = format!("/collections/{}/items", collection_id);
        self.get_request(params, extra_params)
    }

    /// Retreive top-level items for a given collection
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection_top_items = z.get_collection_top_items("AYVWED", "B8ZNE3GH", None);
    /// ```
    fn get_collection_top_items<I: Into<Option<&'a str>>>(
        &self,
        collection_id: &'a str,
        item_id: &'a str,
        extra_params: I,
    ) -> Request<&'static str> {
        let params = format!("/collection/{}/items/{}", collection_id, item_id);
        self.get_request(params, extra_params)
    }

}
