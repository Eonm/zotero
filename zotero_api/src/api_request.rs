use http::{header::AUTHORIZATION, Request, header::HeaderValue};
use serde::Serialize;
use bytes::Bytes;
use url::form_urlencoded::byte_serialize;

pub trait ZoteroApi<'a> {
    fn get_base_url(&self) -> String;
    fn get_id(&self) -> &'a str;
    fn get_api_key(&self) -> &'a str;

    fn request<I: Into<Option<&'a str>>, T: Serialize>(
        &self,
        method: &str,
        params: String,
        extra_params: I,
        data: Option<&T>
    ) -> Request<Bytes> {
        let extra_params: Option<&str> = extra_params.into();
        Request::builder()
            .method(method)
            .uri(match extra_params {
                None => format!("{}{}", self.get_base_url(), params),
                Some(extra_params) => format!("{}{}?{}", self.get_base_url(), params, extra_params),
            })
            .header(AUTHORIZATION, format!("Bearer {}", self.get_api_key()))
            .body(Bytes::from(match data {
                Some(data) => serde_json::to_vec(data).unwrap(),
                None => Vec::new(),
            })).unwrap()
    }

    /// Generate Api request to retrieve key information.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let key_info_request = z.get_api_key_info("bZARysJ579K5SdmYuaAJ");
    /// ```
    fn get_api_key_info<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<Bytes>  {
        let params = format!("/keys/{}", self.get_api_key());
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive specific item in the library by it's ID.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let item_request = z.get_item("B8ZNE3GH", None);
    /// ```
    fn get_item<I: Into<Option<&'a str>>>(
        &self,
        item_id: &'a str,
        extra_params: I,
    ) -> Request<Bytes> {
        let params: String = format!("/items/{}", item_id);
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive all items in the library, excluding trashed items.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let items_request = z.get_items(None);
    /// ```
    fn get_items<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<Bytes> {
        let params = "/items".to_string();
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive all child items of a specific item
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let items_request = z.get_child_items("B8ZNE3GH", None);
    /// ```
    fn get_child_items<I: Into<Option<&'a str>>>(
        &self,
        item_id: &'a str,
        extra_params: I,
    ) -> Request<Bytes> {
        let params: String = format!("/items/{}/children", item_id);
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive top-level items in the library, excluding trashed items.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let top_items_request = z.get_top_items(None);
    /// ```
    fn get_top_items<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<Bytes> {
        let params = "/items/top".to_string();
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive items in the trash.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let trashed_items_request = z.get_trashed_items(None);
    /// ```
    fn get_trashed_items<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<Bytes> {
        let params = "/items/trash".to_string();
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive items in "My publications".
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let my_publications_request = z.get_publications(None);
    /// ```
    fn get_publications<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<Bytes> {
        let params = "/publications/items".to_string();
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive a collection by it's id.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection_request = z.get_collection("AYVWED", None);
    /// ```
    fn get_collection<I: Into<Option<&'a str>>>(
        &self,
        collection_id: &'a str,
        extra_params: I,
    ) -> Request<Bytes> {
        let params = format!("/collections/{}", collection_id);
        self.request::<_,()>("GET", params, extra_params, None)
    }


    /// Generate Api request to retreive all collections.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collections_request = z.get_collections(None);
    /// ```
    fn get_collections<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<Bytes> {
        let params = "/collections".to_string();
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive top level collections.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let top_collections_request = z.get_top_collections(None);
    /// ```
    fn get_top_collections<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Request<Bytes> {
        let params = "/collections/top".to_string();
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive all items for a given collection
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection_items_request = z.get_collection_items("AYVWED", None);
    /// ```
    fn get_collection_items<I: Into<Option<&'a str>>>(
        &self,
        collection_id: &'a str,
        extra_params: I,
    ) -> Request<Bytes> {
        let params = format!("/collections/{}/items", collection_id);
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to retreive top-level items for a given collection
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection_top_items_request = z.get_collection_top_items("AYVWED", "B8ZNE3GH", None);
    /// ```
    fn get_collection_top_items<I: Into<Option<&'a str>>>(
        &self,
        collection_id: &'a str,
        item_id: &'a str,
        extra_params: I,
    ) -> Request<Bytes> {
        let params = format!("/collection/{}/items/{}", collection_id, item_id);
        self.request::<_,()>("GET", params, extra_params, None)
    }

    /// Generate Api request to delete a Zotero item.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let delete_req = z.delete_item("PJTUB2WE", "2050");
    /// ```
    fn delete_item<S: AsRef<str> + std::fmt::Display>(
        &self,
        item_key: S,
        last_version: S,
    ) -> Request<Bytes> {
        let params = format!("items/{}", item_key);
        let mut req = self.request::<_,()>("DELETE", params, None, None);
        req.headers_mut().insert("If-Unmodified-Since-Version", HeaderValue::from_str(&last_version.to_string()).unwrap());
        req
    }

    /// Generate Api request to delete a Zotero item.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let delete_req = z.delete_items(vec!["PJTUB2WE", "YXT5PJU9"], "2050");
    /// ```
    fn delete_items<S: AsRef<str> + std::fmt::Display>(
        &self,
        items_keys: Vec<S>,
        last_version: S,
    ) -> Request<Bytes> {
        let params = format!(
            "items/{}",
            items_keys
                .iter()
                .map(|elem| elem.as_ref())
                .collect::<Vec<&str>>()
                .join(" || ")
        );
        let mut req = self.request::<_,()>("DELETE", params, None, None);
        req.headers_mut().insert("If-Unmodified-Since-Version", HeaderValue::from_str(&last_version.to_string()).unwrap());
        req
    }

    /// Generate Api request to delete a Zotero collection.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let delete_req = z.delete_collection("TYQDGEZR", "2050");
    /// ```
    fn delete_collection<S: AsRef<str> + std::fmt::Display>(
        &self,
        item_key: S,
        last_version: S,
    ) -> Request<Bytes> {
        let params = format!("collections/{}", item_key);
        let mut req = self.request::<_,()>("DELETE", params, None, None);
        req.headers_mut().insert("If-Unmodified-Since-Version", HeaderValue::from_str(&last_version.to_string()).unwrap());
        req
    }

    /// Generate Api request to delete a Zotero search.
    fn delete_search<S: AsRef<str> + std::fmt::Display>(
        &self,
        search_key: S,
        last_version: S,
    ) -> Request<Bytes> {
        let params = format!("searches?searchKey={}", search_key);
        let mut req = self.request::<_,()>("DELETE", params, None, None);
        req.headers_mut().insert("If-Unmodified-Since-Version", HeaderValue::from_str(&last_version.to_string()).unwrap());
        req
    }

    /// Generate Api request to delete a Zotero tag.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let delete_req = z.delete_tag("history", "2050");
    /// ```
    fn delete_tag<S: AsRef<str> + std::fmt::Display>(
        &self,
        tag_key: S,
        last_version: S,
    ) -> Request<Bytes> {
        let params = format!("tags?tag={}", tag_key);
        let mut req = self.request::<_,()>("DELETE", params, None, None);
        req.headers_mut().insert("If-Unmodified-Since-Version", HeaderValue::from_str(&last_version.to_string()).unwrap());
        req
    }

    /// Generate Api request to delete multiple Zotero tags.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Delete;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let delete_req = z.delete_tags(vec!["history", "philosophy", "art"], "2050");
    /// ```
    fn delete_tags<S: AsRef<str> + std::fmt::Display>(
        &self,
        tags_keys: Vec<S>,
        last_version: S,
    ) -> Request<Bytes> {
        let params = format!(
            "tags?tag={}",
            tags_keys
                .iter()
                .map(|elem| byte_serialize(elem.as_ref().as_bytes()).collect())
                .collect::<Vec<String>>()
                .join(" || ")
        );
        let mut req = self.request::<_,()>("DELETE", params, None, None);
        req.headers_mut().insert("If-Unmodified-Since-Version", HeaderValue::from_str(&last_version.to_string()).unwrap());
        req
    }

    /// Generate Api request to create an item
    fn create_new_item<T: Serialize>(&self, item: T) -> Request<Bytes> {
        let params = "/items".to_string();
        self.request("POST", params, None, Some(&vec![&item]))
    }

    /// Generate Api request to create multiple items
    fn create_new_items<T: Serialize>(&self, item: Vec<T>) -> Request<Bytes> {
        let params = "/items".to_string();
        self.request("POST", params, None, Some(&item))
    }

    /// Generate Api request to create new collection
    fn create_new_collection<T: Serialize>(&self, item: T) -> Request<Bytes> {
        let params = "/collections".to_string();
        self.request("POST", params, None, Some(&vec![&item]))
    }

    /// Generate Api request to create new collections
    fn create_new_collections<T: Serialize>(
        &self,
        item: Vec<T>,
    ) -> Request<Bytes> {
        let params = "/collections".to_string();
        self.request("POST", params, None, Some(&item))
    }

    /// Generate Api request to update a zotero item.
    /// ```no_run
    /// # use zotero_api::ZoteroInit;
    /// # use zotero_api::Patch;
    /// # use zotero_api::Get;
    /// # use zotero_data::item::ItemType;
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
    ) -> Request<Bytes> {
        let params = format!("/items/{}", item_key);
        self.request("PATCH", params, None, Some(&item_data))
    }
}
