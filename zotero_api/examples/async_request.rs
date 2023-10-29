use tokio;
use zotero_api::{Zotero, ZoteroApi, ZoteroApiAsyncExecutor, ZoteroApiError};
use zotero_data::item::Item;

#[tokio::main]
async fn main() {
    let zotero_api = Zotero::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    let result: Result<Vec<Item>, ZoteroApiError> =
        zotero_api.get_items(None).execute(&zotero_api).await;
    match result {
        Ok(items) => println!("{}", items.len()),
        Err(err) => println!("{err:?}"),
    };
}
