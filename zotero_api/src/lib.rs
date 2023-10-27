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
//! use zotero_api::Zotero;
//! use zotero_api::Post;
//! use zotero_data::item::{BookData, BookDataBuilder, Creator, CreatorBuilder};
//!
//! let z = Zotero::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
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
//! use zotero_api::Zotero;
//! use zotero_api::Patch;
//! use zotero_api::Get;
//! use zotero_data::item::ItemType;
//!
//! let z = Zotero::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
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
mod reqwest_impl;

use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

pub use crate::api_request::ZoteroApi;

#[derive(Debug, Error)]
pub enum ZoteroApiError {
    #[error("Error generating Http request: {0}")]
    RequestCreationError(String),
    #[error("Request Error: {0}")]
    RequestError(String),
    #[error("Parse Response Error: {0}")]
    ParseResponseError(String),
}

pub trait ZoteroApiExecutor {
    fn execute<'a, T: Deserialize<'a>, Z: ZoteroApi<'a>>(self, zotero_api: &Z) -> Result<T, ZoteroApiError>;
}

#[async_trait]
pub trait ZoteroApiAsyncExecutor {
    async fn execute<'a, T: Deserialize<'a>, Z: ZoteroApi<'a> + std::marker::Sync>(self, zotero_api: &Z) -> Result<T, ZoteroApiError>;
}

/// A struct representing a Zotero client.
#[derive(Debug, PartialEq)]
pub struct Zotero<'a> {
    pub library_type: LibraryType<'a>,
}

impl<'a> Zotero<'a> {
    /// Create a Zotero client for a group library. According to group policy API key might be optional.
    /// ```rust
    /// # use zotero_api::Zotero;
    /// // With an API key
    /// let z = Zotero::set_group("123456789", "bZARysJ579K5SdmYuaAJ");
    ///
    /// // Without API key
    /// let z = Zotero::set_group("123456789", None);
    /// ```
    pub fn set_group<S: Into<Option<&'a str>>>(group_id: &'a str, api_key: S) -> Zotero<'a> {
        Zotero {
            library_type: LibraryType::group(group_id, api_key),
        }
    }

    /// Create a Zotero client for a user library.
    /// ```rust
    /// # use zotero_api::Zotero;
    /// let z = Zotero::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
    /// ```
    pub fn set_user(user_id: &'a str, api_key: &'a str) -> Zotero<'a> {
        Zotero {
            library_type: LibraryType::user(user_id, api_key),
        }
    }
}

impl<'a> ZoteroApi<'a> for Zotero<'a> {
    fn get_id(&self) -> &'a str {
        self.library_type.get_id()
    }

    fn get_api_key(&self) -> Option<&'a str> {
        self.library_type.get_api_key()
    }

    fn get_base_url(&self) -> String {
        self.library_type.get_base_url()
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

        let result_0 = Zotero::set_group("123456", None);
        assert_eq!(result_0, expected_struct0);

        let expected_struct1 = Zotero {
            library_type: LibraryType::UserLibrary {
                user_id: "123456",
                api_key: "abc",
            },
        };

        let result_1 = Zotero::set_user("123456", "abc");
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

        let result_0 = Zotero::set_group("456", Some("123"));

        assert_eq!(result_0, expected_struct0);

        let expected_struct1 = Zotero {
            library_type: LibraryType::GroupLibrary {
                group_id: "456",
                api_key: Some("123"),
            },
        };

        let result_1 = Zotero::set_group("456", "123");

        assert_eq!(result_1, expected_struct1);

        let expected_struct2 = Zotero {
            library_type: LibraryType::GroupLibrary {
                group_id: "456",
                api_key: None,
            },
        };

        let result_2 = Zotero::set_group("456", None);

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
