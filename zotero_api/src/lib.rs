//! # Zotero Api
//!
//! [![Build Status](https://travis-ci.org/Eonm/zotero.svg?branch=master)](https://travis-ci.org/Eonm/zotero)
//! [![Coverage Status](https://coveralls.io/repos/github/Eonm/zotero/badge.svg?branch=master)](https://coveralls.io/github/Eonm/zotero?branch=master)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/Eonm/zotero/issues)
//!
//! [API documentation](https://docs.rs/zotero/)
//!
//! ## Creating items and collections
//!
//! ```no_run
//! use zotero_api::ZoteroInit;
//! use zotero_api::Post;
//! use zotero_data::item::{BookData, BookDataBuilder, Creator, CreatorBuilder};
//!
//! let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
//!
//! let creators : Vec<Creator> = vec![
//!     CreatorBuilder::default()
//!         .creator_type("author")
//!         .first_name("John")
//!         .last_name("Doe")
//!         .build()
//!         .unwrap()
//! ];
//!
//! let new_book : BookData = BookDataBuilder::default()
//!     .title("Sample_2")
//!     .creators(creators)
//!     .item_type("book")
//!     .build()
//!     .unwrap();
//!
//! z.create_new_item(new_book);
//! ```
//!
//! ## Updating items and collections
//!
//! ```no_run
//! use zotero_api::ZoteroInit;
//! use zotero_api::Patch;
//! use zotero_api::Get;
//! use zotero_data::item::ItemType;
//!
//! let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
//! let item = z.get_item("Q8GNE36F", None);
//! if let Ok(mut result) = item {
//!     if let ItemType::Book(bookdata) = &mut result.data {
//!         bookdata.title = "A new title".to_string();
//!         bookdata.publisher = "A new publisher".to_string();
//!         z.update_item(&bookdata.key, &bookdata);
//!     };
//!
//!     println!("{:?}", serde_json::to_string(&result.data));
//! };
//! ```

mod api_request;
mod consts;

pub use crate::api_request::ZoteroApi;

/// A struct representing a Zotero client.
#[derive(Debug, PartialEq)]
pub struct Zotero<'a> {
    pub library_type: LibraryType<'a>,
}

impl<'a> Zotero<'a> {
//    fn get_request<S: AsRef<str> + std::fmt::Display>(
//        &self,
//        params: S,
//        extra_params: Option<&str>,
//    ) -> Result<Value, Box<dyn error::Error>> {
//        let base_url = self.library_type.get_base_url();
//
//        let url = match extra_params {
//            None => format!("{}{}", base_url, params),
//            Some(extra_par) => format!("{}{}?{}", base_url, params, extra_par),
//        };
//
//        let client = reqwest::blocking::Client::new();
//
//        let res = match &self.library_type.get_api_key() {
//            Some(key) => client.get(&url).bearer_auth(key).send()?,
//            None => client.get(&url).send()?,
//        };
//
//        match &res.headers().get(header::LINK) {
//            None => Ok(res.json()?),
//            Some(v) => {
//                let link: Link = v.to_str().unwrap().parse()?;
//                let mut next: Option<LinkValue> = None;
//                for l in link.values() {
//                    match l.rel() {
//                        None => {}
//                        Some(reltypes) => {
//                            if reltypes.contains(&RelationType::Next) {
//                                next = Some(l.clone());
//                            }
//                        }
//                    }
//                }
//                let mut chain_responses: Vec<Value> = res.json()?;
//                while next.is_some() {
//                    let next_link = next.unwrap();
//                    next = None;
//                    let res = match &self.library_type.get_api_key() {
//                        Some(key) => client.get(next_link.link()).bearer_auth(key).send()?,
//                        None => client.get(next_link.link()).send()?,
//                    };
//                    match &res.headers().get(header::LINK) {
//                        None => {
//                            next = None;
//                        }
//                        Some(v) => {
//                            let link: Link = v.to_str().unwrap().parse()?;
//                            for l in link.values() {
//                                match l.rel() {
//                                    None => {}
//                                    Some(reltypes) => {
//                                        if reltypes.contains(&RelationType::Next) {
//                                            next = Some(l.clone());
//                                        }
//                                    }
//                                }
//                            }
//                        }
//                    }
//                    let mut v: Vec<Value> = res.json()?;
//                    chain_responses.append(&mut v);
//                }
//                Ok(Value::Array(chain_responses))
//            }
//        }
//    }
}

impl<'a> ZoteroApi<'a> for Zotero<'a> {
    fn get_id(&self) -> &'a str {
        self.library_type.get_id()
    }

    fn get_api_key(&self) -> &'a str {
        self.library_type.get_api_key().unwrap()
    }

    fn get_base_url(&self) -> String {
        self.library_type.get_base_url()
    }
}

//impl<'a> Patch<'a> for Zotero<'a> {
//    fn patch_request<T: Serialize>(
//        &self,
//        params: &str,
//        json_body: T,
//    ) -> Result<Value, Box<dyn error::Error>> {
//        let url = format!("{}{}", self.library_type.get_base_url(), params);
//
//        let client = reqwest::blocking::Client::new();
//        let res = client
//            .patch(url)
//            .bearer_auth(self.library_type.get_api_key().unwrap())
//            .json(&json_body)
//            .send()?;
//
//        Ok(res.json()?)
//    }
//
//    fn get_id(&self) -> &'a str {
//        self.library_type.get_id()
//    }
//}


/// Create a Zotero client for a group library or a user library.
/// ```no_run
/// # use zotero_api::{ZoteroInit, Zotero};
/// # use zotero_api::Get;
/// let z : Zotero = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
///
/// // Perform some operation with Zotero
///
/// let items = z.get_items(None);
///
/// for item in items {
///     println!("{:#?}", item)
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct ZoteroInit<'a> {
    pub library_type: Option<LibraryType<'a>>,
}

impl<'a> ZoteroInit<'a> {
    /// Create a Zotero client for a group library. According to group policy API key might be optional.
    /// ```rust
    /// # use zotero_api::ZoteroInit;
    /// // With an API key
    /// let z = ZoteroInit::set_group("123456789", "bZARysJ579K5SdmYuaAJ");
    ///
    /// // Without API key
    /// let z = ZoteroInit::set_group("123456789", None);
    /// ```
    pub fn set_group<S: Into<Option<&'a str>>>(group_id: &'a str, api_key: S) -> Zotero<'a> {
        Zotero {
            library_type: LibraryType::group(group_id, api_key),
        }
    }

    /// Create a Zotero client for a user library.
    /// ```rust
    /// # use zotero_api::ZoteroInit;
    /// let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// ```
    pub fn set_user(user_id: &'a str, api_key: &'a str) -> Zotero<'a> {
        Zotero {
            library_type: LibraryType::user(user_id, api_key),
        }
    }
}

/// Either a User library or a Group library
#[derive(Debug, PartialEq)]
pub enum LibraryType<'a> {
    UserLibrary {
        user_id: &'a str,
        api_key: &'a str,
    },
    GroupLibrary {
        group_id: &'a str,
        api_key: Option<&'a str>,
    },
}

impl<'a> LibraryType<'a> {
    fn get_id(&self) -> &'a str {
        match self {
            LibraryType::UserLibrary { user_id, .. } => user_id,
            LibraryType::GroupLibrary { group_id, .. } => group_id,
        }
    }

    fn get_api_key(&self) -> Option<&'a str> {
        match self {
            LibraryType::UserLibrary { api_key, .. } => Some(*api_key),
            LibraryType::GroupLibrary { api_key, .. } => api_key.to_owned(),
        }
    }

    fn get_base_url(&self) -> String {
        match self {
            LibraryType::UserLibrary { .. } => {
                format!("{}{}{}", consts::ZOTERO_BASE_URL, "users/", self.get_id())
            }
            LibraryType::GroupLibrary { .. } => {
                format!("{}{}{}", consts::ZOTERO_BASE_URL, "groups/", self.get_id())
            }
        }
    }

    fn group<S: Into<Option<&'a str>>>(group_id: &'a str, api_key: S) -> LibraryType<'a> {
        LibraryType::GroupLibrary {
            group_id,
            api_key: api_key.into(),
        }
    }

    fn user(user_id: &'a str, api_key: &'a str) -> LibraryType<'a> {
        LibraryType::UserLibrary { user_id, api_key }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_zotero() {
        let expected_struct0 = Zotero {
            library_type: LibraryType::GroupLibrary {
                group_id: "123456",
                api_key: None,
            },
        };

        let result_0 = ZoteroInit::set_group("123456", None);
        assert_eq!(result_0, expected_struct0);

        let expected_struct1 = Zotero {
            library_type: LibraryType::UserLibrary {
                user_id: "123456",
                api_key: "abc",
            },
        };

        let result_1 = ZoteroInit::set_user("123456", "abc");
        assert_eq!(result_1, expected_struct1);
    }

    #[test]
    fn test_group_new() {
        let expected_struct0 = Zotero {
            library_type: LibraryType::GroupLibrary {
                group_id: "456",
                api_key: Some("123"),
            },
        };

        let result_0 = ZoteroInit::set_group("456", Some("123"));

        assert_eq!(result_0, expected_struct0);

        let expected_struct1 = Zotero {
            library_type: LibraryType::GroupLibrary {
                group_id: "456",
                api_key: Some("123"),
            },
        };

        let result_1 = ZoteroInit::set_group("456", "123");

        assert_eq!(result_1, expected_struct1);

        let expected_struct2 = Zotero {
            library_type: LibraryType::GroupLibrary {
                group_id: "456",
                api_key: None,
            },
        };

        let result_2 = ZoteroInit::set_group("456", None);

        assert_eq!(result_2, expected_struct2);
    }

    #[test]
    fn test_library_type() {
        let user_library = LibraryType::UserLibrary {
            user_id: "123456789",
            api_key: "abcdef",
        };

        assert_eq!(user_library.get_id(), "123456789");
        assert_eq!(user_library.get_api_key(), Some("abcdef"));
        assert_eq!(
            user_library.get_base_url(),
            "https://api.zotero.org/users/123456789"
        );

        let group_id = LibraryType::GroupLibrary {
            group_id: "123456789",
            api_key: Some("abcdef"),
        };

        assert_eq!(group_id.get_id(), "123456789");
        assert_eq!(group_id.get_api_key(), Some("abcdef"));
        assert_eq!(
            group_id.get_base_url(),
            "https://api.zotero.org/groups/123456789"
        );
    }
}
