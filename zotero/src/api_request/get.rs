use crate::data_structure::collection::Collection;
use crate::data_structure::item::Item;
use serde_json::value::Value;
use std::error;

/// Perform get operations on Zotero items and collections.
/// ```no_run
/// use zotero::ZoteroInit;
/// use zotero::Get;
///
/// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
/// let items = z.get_items(None);
/// ```
pub trait Get<'a> {
    fn get_request<S: AsRef<str> + std::fmt::Display>(
        &self,
        params: S,
        extra_params: Option<&str>,
    ) -> Result<Value, Box<dyn error::Error>>;
    fn get_id(&self) -> &'a str;
    fn get_api_key(&self) -> &'a str;

    /// Retreive information about an api key and it's privileges.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection = z.get_api_key_info("bZARysJ579K5SdmYuaAJ");
    /// ```
    fn get_api_key_info<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Result<Value, Box<dyn error::Error>> {
        let params = format!("/keys/{}", self.get_api_key());
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive specific item in the library by it's ID.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let items = z.get_item("B8ZNE3GH", None);
    /// ```
    fn get_item<I: Into<Option<&'a str>>>(
        &self,
        item_id: &'a str,
        extra_params: I,
    ) -> Result<Item, Box<dyn error::Error>> {
        let params: String = format!("/items/{}", item_id);
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive all items in the library, excluding trashed items.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let items = z.get_items(None);
    /// ```
    fn get_items<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Result<Vec<Item>, Box<dyn error::Error>> {
        let params = "/items".to_string();
        let response = self.get_request(&params, extra_params.into())?;

        Ok(serde_json::from_value(response)?)
    }

    /// Retreive all child items of a specific item
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let items = z.get_child_items("B8ZNE3GH", None);
    /// ```
    fn get_child_items<I: Into<Option<&'a str>>>(
        &self,
        item_id: &'a str,
        extra_params: I,
    ) -> Result<Vec<Item>, Box<dyn error::Error>> {
        let params: String = format!("/items/{}/children", item_id);
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive top-level items in the library, excluding trashed items.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let top_items = z.get_top_items(None);
    /// ```
    fn get_top_items<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Result<Vec<Item>, Box<dyn error::Error>> {
        let params = "/items/top".to_string();
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive items in the trash.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let trashed_items = z.get_trashed_items(None);
    /// ```
    fn get_trashed_items<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Result<Vec<Item>, Box<dyn error::Error>> {
        let params = "/items/trash".to_string();
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive items in "My publications".
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let my_publications = z.get_publications(None);
    /// ```
    fn get_publications<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Result<Vec<Item>, Box<dyn error::Error>> {
        let params = "/publications/items".to_string();
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive a collection by it's id.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection = z.get_collection("AYVWED", None);
    /// ```
    fn get_collection<I: Into<Option<&'a str>>>(
        &self,
        collection_id: &'a str,
        extra_params: I,
    ) -> Result<Collection, Box<dyn error::Error>> {
        let params = format!("/collections/{}", collection_id);
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive a collection by it's name.    
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection = z.get_collection_by_name("My collection", None);
    /// ```
    fn get_collection_by_name<I: Into<Option<&'a str>>>(
        &self,
        collection_name: &'a str,
        extra_params: I,
    ) -> Result<Option<Collection>, Box<dyn error::Error>> {
        let collections = self.get_collections(extra_params)?;
        let filtered_collections = collections
            .into_iter()
            .filter(|collection| collection.data.name == collection_name)
            .collect::<Vec<Collection>>();

        if !filtered_collections.is_empty() {
            Ok(Some(filtered_collections[0].clone()))
        } else {
            Ok(None)
        }
    }

    /// Retreive a set of collections by their names.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collections = z.get_collections_by_names(vec!("My collection", "My other collection"), None);
    /// ```
    fn get_collections_by_names<I: Into<Option<&'a str>>>(
        &self,
        collection_names: Vec<&'a str>,
        extra_params: I,
    ) -> Result<Option<Vec<Collection>>, Box<dyn error::Error>> {
        let collections = self.get_collections(extra_params)?;
        let filtered_collections = collections
            .into_iter()
            .filter(|collection| {
                let collection_data_name: &str = &collection.data.name;
                collection_names.contains(&collection_data_name)
            })
            .collect::<Vec<Collection>>();

        if !filtered_collections.is_empty() {
            Ok(Some(filtered_collections))
        } else {
            Ok(None)
        }
    }

    /// Retreive all collections.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collections = z.get_collections(None);
    /// ```
    fn get_collections<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Result<Vec<Collection>, Box<dyn error::Error>> {
        let params = "/collections".to_string();
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive top level collections.
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let top_collections = z.get_top_collections(None);
    /// ```
    fn get_top_collections<I: Into<Option<&'a str>>>(
        &self,
        extra_params: I,
    ) -> Result<Vec<Collection>, Box<dyn error::Error>> {
        let params = "/collections/top".to_string();
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive all items for a given collection
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection_items = z.get_collection_items("AYVWED", None);
    /// ```
    fn get_collection_items<I: Into<Option<&'a str>>>(
        &self,
        collection_id: &'a str,
        extra_params: I,
    ) -> Result<Vec<Item>, Box<dyn error::Error>> {
        let params = format!("/collections/{}/items", collection_id);
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive top-level items for a given collection
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let collection_top_items = z.get_collection_top_items("AYVWED", "B8ZNE3GH", None);
    /// ```
    fn get_collection_top_items<I: Into<Option<&'a str>>>(
        &self,
        collection_id: &'a str,
        item_id: &'a str,
        extra_params: I,
    ) -> Result<Vec<Item>, Box<dyn error::Error>> {
        let params = format!("/collection/{}/items/{}", collection_id, item_id);
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    /// Retreive up to 50 items by their ids
    /// ```no_run
    /// # use zotero::ZoteroInit;
    /// # use zotero::Get;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// let items = z.get_subset(vec!("AYVWED", "B8ZNE3GH"), None);
    /// ```
    fn get_subset<I: Into<Option<&'a str>>>(
        &self,
        item_ids: Vec<&'a str>,
        extra_params: I,
    ) -> Result<Vec<Item>, Box<dyn error::Error>> {
        let params = format!(
            "/items?itemKey={}",
            item_ids
                .iter()
                .map(|elem| elem.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        let response = self.get_request(&params, extra_params.into())?;
        Ok(serde_json::from_value(response)?)
    }

    // /// Retreive notes.
    // /// ```no_run
    // /// # use zotero::ZoteroInit;
    // /// # use zotero::Get;
    // /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    // /// let items = z.get_all_notes();
    // /// ```
    // fn get_all_notes(&self) -> Result<Vec<Item>, Box<dyn error::Error>> {
    //     let params = format!("/items/");
    //     let response = self.get_request(&params, Some("itemType=note"))?;
    //     Ok(serde_json::from_value(response)?)
    // }

    // ///Retreive all attachment.
    // /// ```no_run
    // /// # use zotero::ZoteroInit;
    // /// # use zotero::Get;
    // /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    // /// let items = z.get_all_attachments();
    // /// ```
    // fn get_all_attachments(&self) -> Result<Vec<Item>, Box<dyn error::Error>> {
    //     let params = format!("/items/");
    //     let response = self.get_request(&params, Some("itemType=attachment"))?;
    //     Ok(serde_json::from_value(response)?)
    // }
}
