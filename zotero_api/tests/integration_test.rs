use zotero_api::{Zotero, ZoteroApi};
#[allow(clippy::assertions_on_constants)]
use zotero_data::item::{BookData, BookDataBuilder};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_item() {
        let book: BookData = BookDataBuilder::default().title("e").build().unwrap();

        assert!(book.title == "e")
    }

    #[test]
    fn creating_zotero_user_client() {
        let _z = Zotero::set_user("123456789", "abcdefghij");
        assert!(true)
    }

    #[test]
    fn creating_zotero_group_client() {
        let _z = Zotero::set_group("123456789", None);
        assert!(true)
    }

    #[test]
    fn get_operation_for_user() {
        let z = Zotero::set_user("123456789", "abcdefgh");
        let req = z.get_collection("ABREZSE", None);
        assert_eq!(req.uri(), "https://api.zotero.org/users/123456789/collections/ABREZSE")
    }

    #[test]
    fn get_operation_for_group() {
        let z = Zotero::set_group("123456789", None);
        let req =  z.get_collection("ABREZSE", None);
        assert_eq!(req.uri(), "https://api.zotero.org/groups/123456789/collections/ABREZSE")
    }

    #[test]
    fn delete_operation_for_user() {
        let z = Zotero::set_user("123456789", "abcdefgh");
        let req =  z.delete_item("ABREZSE", "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_operation_for_group_no_api_key() {
        let z = Zotero::set_group("123456789", None);
        let req = z.delete_item("ABREZSE", "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), false);
    }

    #[test]
    fn delete_operation_for_group() {
        let z = Zotero::set_group("123456789", "abcdefgh");
        let req = z.delete_item("ABREZSE", "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_collection_for_group() {
        let z = Zotero::set_group("123456789", "abcdefgh");
        let req = z.delete_collection("ABREZSE", "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_collection_for_user() {
        let z = Zotero::set_user("123456789", "abcdefgh");
        let req = z.delete_collection("ABREZSE", "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_items_for_user() {
        let z = Zotero::set_user("123456789", "abcdefgh");
        let req = z.delete_items(vec!["ABREZSE"], "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_items_for_group() {
        let z = Zotero::set_group("123456789", "abcdefgh");
        let req = z.delete_items(vec!["ABREZSE"], "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_tag_for_group() {
        let z = Zotero::set_group("123456789", "abcdefgh");
        let req = z.delete_tag("ABREZSE", "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_tag_for_user() {
        let z = Zotero::set_user("123456789", "abcdefgh");
        let req = z.delete_tag("ABREZSE", "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_tags_for_group() {
        let z = Zotero::set_group("123456789", "abcdefgh");
        let req = z.delete_tags(vec!["ABREZSE"], "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_tags_for_user() {
        let z = Zotero::set_user("123456789", "abcdefgh");
        let req = z.delete_tags(vec!["ABREZSE"], "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_search_for_user() {
        let z = Zotero::set_user("123456789", "abcdefgh");
        let req = z.delete_search("ABREZSE", "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }

    #[test]
    fn delete_search_for_group() {
        let z = Zotero::set_group("123456789", "abcdefgh");
        let req = z.delete_search("ABREZSE", "2050");
        assert_eq!(req.method(), "DELETE");
        assert_eq!(req.headers().contains_key("Authorization"), true);
    }
}
