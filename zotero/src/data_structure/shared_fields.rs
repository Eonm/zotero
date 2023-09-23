use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Library {
    r#type: String,
    pub id: usize,
    pub name: String,
    pub links: Links,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Links {
    #[serde(alias = "self")]
    pub self_link: Option<Link>,
    pub alternate: Link,
    // Only for collections
    pub up: Option<Link>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Link {
    pub href: String,
    pub r#type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Tag {
    pub tag: String
}

pub trait Identifier {
    fn key(&self) -> &String;
}

pub trait ItemCommon {
    fn title(&self) -> &str;

    fn tags(&self) -> &Vec<Tag>;

    fn key(&self) -> &str;

    fn has_tag(&self, tag: String) -> bool {
      self.tags().iter().any(|t| t.tag == tag)
    }
}

#[cfg(test)]
mod test_shared_fields_deserialization {
    use super::*;
    #[test]
    fn link_deserialization() {
        let expected_output = Link {
            href: "https://www.zotero.org/john.doe".into(),
            r#type: "text/html".into(),
        };

        let input = r#"
            {     
                "href": "https://www.zotero.org/john.doe",
                "type": "text/html"     
            }
        "#;

        let result = serde_json::from_str::<Link>(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn library_deserialization() {
        let link_0 = Link {
            href: "https://api.zotero.org/users/123456789/items/Q8GNE36F".into(),
            r#type: "application/json".into(),
        };

        let link_1 = Link {
            href: "https://www.zotero.org/john.doe/items/Q8GNE36F".into(),
            r#type: "text/html".into(),
        };

        let expected_output = Links {
            self_link: Some(link_0),
            alternate: link_1,
            up: None,
        };

        let input = r#"
            {
                "self": {
                    "href": "https://api.zotero.org/users/123456789/items/Q8GNE36F",
                    "type": "application/json"
                },
                "alternate": {
                    "href": "https://www.zotero.org/john.doe/items/Q8GNE36F",
                    "type": "text/html"
                }
            }
        "#;

        let result = serde_json::from_str::<Links>(input).unwrap();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn links_deserialization() {
        let link_0 = Link {
            href: "https://www.zotero.org/john.doe".into(),
            r#type: "text/html".into(),
        };

        let expected_output = Library {
            r#type: "user".into(),
            id: 123456789,
            name: "john.doe".into(),
            links: Links {
                alternate: link_0,
                self_link: None,
                up: None,
            },
        };

        let input = r#"
            {
                "type": "user",
                "id": 123456789,
                "name": "john.doe",
                "links": {
                    "alternate": {
                        "href": "https://www.zotero.org/john.doe",
                        "type": "text/html"
                    }
                }
        }
        "#;

        let result = serde_json::from_str::<Library>(input).unwrap();
        assert_eq!(result, expected_output);
    }
}
