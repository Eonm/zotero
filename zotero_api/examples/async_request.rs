use tokio;
use zotero_api::{Zotero, ZoteroApi, ZoteroApiAsyncExecutor};
use zotero_data::item::Item;

#[tokio::main]
async fn main() {
    let zotero_api = Zotero::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    let get_items_req = zotero_api.get_items(None);
    let items: Vec<Item> = get_items_req.execute(&zotero_api).await.unwrap();
    println!("{:#?}", items.len());
}
