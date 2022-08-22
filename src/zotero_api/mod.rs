use crate::api_request::delete::Delete;
use crate::api_request::get::Get;
use crate::api_request::patch::Patch;
use crate::api_request::post::Post;
use crate::consts;
use std::error;

use serde::Serialize;
use serde_json::Value;

/// A struct representing a Zotero client.
#[derive(Debug, PartialEq)]
pub struct Zotero<'a> {
    pub library_type: LibraryType<'a>,
}

impl<'a> Get<'a> for Zotero<'a> {
    fn get_request<S: AsRef<str> + std::fmt::Display>(
        &self,
        params: S,
        extra_params: Option<&str>,
    ) -> Result<Value, Box<dyn error::Error>> {
        let base_url = self.library_type.get_base_url();

        let url = match extra_params {
            None => format!("{}{}", base_url, params),
            Some(extra_par) => format!("{}{}?{}", base_url, params, extra_par),
        };

        let client = reqwest::blocking::Client::new();

        let mut res = match &self.library_type.get_api_key() {
            Some(key) => {
                client
                    .get(&url)
                    .bearer_auth(key)
                    .send()?
            },
            None => {
                client
                    .get(&url)
                    .send()?
            },
        };

        Ok(res.json()?)
    }

    fn get_id(&self) -> &'a str {
        self.library_type.get_id()
    }

    fn get_api_key(&self) -> &'a str {
        self.library_type.get_api_key().unwrap()
    }
}

impl<'a> Post<'a> for Zotero<'a> {
    fn post_request<T: Serialize, S: AsRef<str> + std::fmt::Display>(
        &self,
        params: S,
        json_body: T,
    ) -> Result<Value, Box<dyn error::Error>> {
        let url = format!("{}{}", self.library_type.get_base_url(), params);

        let client = reqwest::blocking::Client::new();
        let mut res = client
            .post(&url)
            .bearer_auth(&self.library_type.get_api_key().unwrap())
            .json(&json_body)
            .send()?;

        Ok(res.json()?)
    }

    fn get_id(&self) -> &'a str {
        self.library_type.get_id()
    }
}

impl<'a> Patch<'a> for Zotero<'a> {
    fn patch_request<T: Serialize>(
        &self,
        params: &str,
        json_body: T,
    ) -> Result<Value, Box<dyn error::Error>> {
        let url = format!("{}{}", self.library_type.get_base_url(), params);

        let client = reqwest::blocking::Client::new();
        let mut res = client
            .patch(&url)
            .bearer_auth(&self.library_type.get_api_key().unwrap())
            .json(&json_body)
            .send()?;

        Ok(res.json()?)
    }

    fn get_id(&self) -> &'a str {
        self.library_type.get_id()
    }
}

impl<'a> Delete<'a> for Zotero<'a> {
    fn delete_request(
        &self,
        params: &str,
        last_version: &str,
    ) -> Result<Value, Box<dyn error::Error>> {
        let url = format!("{}{}", self.library_type.get_base_url(), params);

        let client = reqwest::blocking::Client::new();
        let mut res = client
            .delete(&url)
            .bearer_auth(&self.library_type.get_api_key().unwrap())
            .header("If-Unmodified-Since-Version", last_version)
            .send()?;

        Ok(res.json()?)
    }

    fn get_id(&self) -> &'a str {
        self.library_type.get_id()
    }
}

/// Create a Zotero client for a group library or a user library.
/// ```no_run
/// # use zotero::{ZoteroInit, Zotero};
/// # use zotero::Get;
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
    /// # use zotero::ZoteroInit;
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
    /// # use zotero::ZoteroInit;
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
                format!("{}{}{}/", consts::ZOTERO_BASE_URL, "users/", self.get_id())
            }
            LibraryType::GroupLibrary { .. } => {
                format!("{}{}{}/", consts::ZOTERO_BASE_URL, "groups/", self.get_id())
            }
        }
    }

    fn group<S: Into<Option<&'a str>>>(group_id: &'a str, api_key: S) -> LibraryType<'a> {
        LibraryType::GroupLibrary {
            group_id: group_id,
            api_key: api_key.into(),
        }
    }

    fn user(user_id: &'a str, api_key: &'a str) -> LibraryType<'a> {
        LibraryType::UserLibrary {
            user_id: user_id,
            api_key: api_key,
        }
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
            "https://api.zotero.org/users/123456789/"
        );

        let group_id = LibraryType::GroupLibrary {
            group_id: "123456789",
            api_key: Some("abcdef"),
        };

        assert_eq!(group_id.get_id(), "123456789");
        assert_eq!(group_id.get_api_key(), Some("abcdef"));
        assert_eq!(
            group_id.get_base_url(),
            "https://api.zotero.org/groups/123456789/"
        );
    }
}
