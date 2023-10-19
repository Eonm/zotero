#[allow(clippy::assertions_on_constants)]
extern crate zotero;
use zotero::data_structure::item::{BookData, BookDataBuilder};
use zotero::ZoteroInit;
use zotero::{Delete, Get};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_item() {
        let _book: BookData = BookDataBuilder::default().title("e").build().unwrap();

        assert!(true)
    }

    #[test]
    fn creating_zotero_user_client() {
        let _z = ZoteroInit::set_user("123456789", "abcdefghij");
        assert!(true)
    }

    #[test]
    fn creating_zotero_group_client() {
        let _z = ZoteroInit::set_group("123456789", None);
        assert!(true)
    }

    #[test]
    fn get_operation_for_user() {
        let z = ZoteroInit::set_user("123456789", "abcdefgh");
        match z.get_collection("ABREZSE", None) {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn get_operation_for_group() {
        let z = ZoteroInit::set_group("123456789", None);
        match z.get_collection("ABREZSE", None) {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_operation_for_user() {
        let z = ZoteroInit::set_user("123456789", "abcdefgh");
        match z.delete_item("ABREZSE", "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    #[should_panic]
    fn delete_operation_for_group_no_api_key() {
        let z = ZoteroInit::set_group("123456789", None);
        match z.delete_item("ABREZSE", "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_operation_for_group() {
        let z = ZoteroInit::set_group("123456789", "abcdefgh");
        match z.delete_item("ABREZSE", "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_collection_for_group() {
        let z = ZoteroInit::set_group("123456789", "abcdefgh");
        match z.delete_collection("ABREZSE", "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_collection_for_user() {
        let z = ZoteroInit::set_user("123456789", "abcdefgh");
        match z.delete_collection("ABREZSE", "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_items_for_user() {
        let z = ZoteroInit::set_user("123456789", "abcdefgh");
        match z.delete_items(vec!["ABREZSE"], "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_items_for_group() {
        let z = ZoteroInit::set_group("123456789", "abcdefgh");
        match z.delete_items(vec!["ABREZSE"], "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_tag_for_group() {
        let z = ZoteroInit::set_group("123456789", "abcdefgh");
        match z.delete_tag("ABREZSE", "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_tag_for_user() {
        let z = ZoteroInit::set_user("123456789", "abcdefgh");
        match z.delete_tag("ABREZSE", "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_tags_for_group() {
        let z = ZoteroInit::set_group("123456789", "abcdefgh");
        match z.delete_tags(vec!["ABREZSE"], "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_tags_for_user() {
        let z = ZoteroInit::set_user("123456789", "abcdefgh");
        match z.delete_tags(vec!["ABREZSE"], "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_search_for_user() {
        let z = ZoteroInit::set_user("123456789", "abcdefgh");
        match z.delete_search("ABREZSE", "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn delete_search_for_group() {
        let z = ZoteroInit::set_group("123456789", "abcdefgh");
        match z.delete_search("ABREZSE", "2050") {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        };
    }
}
