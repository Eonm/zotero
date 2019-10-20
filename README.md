# Zotero

[![Build Status](https://travis-ci.org/Eonm/zotero.svg?branch=master)](https://travis-ci.org/Eonm/zotero)
[![Coverage Status](https://coveralls.io/repos/github/Eonm/zotero/badge.svg?branch=master)](https://coveralls.io/github/Eonm/zotero?branch=master)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/Eonm/zotero/issues)

[API documentation](https://docs.rs/zotero/)

## Creating items and collections

```rust
extern crate zotero;
use zotero::ZoteroInit;
use zotero::Post;
use zotero::data_structure::item::{BookData, BookDataBuilder, Creator, CreatorBuilder};

let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");

let creators : Vec<Creator> = vec![
    CreatorBuilder::default()
        .creator_type("author")
        .first_name("John")
        .last_name("Doe")
        .build()
        .unwrap()
];

let new_book : BookData = BookDataBuilder::default()
    .title("Sample_2")
    .creators(creators)
    .item_type("book")
    .build()
    .unwrap();
    
z.create_new_item(new_book);
```

## Updating items and collections

```rust
extern crate zotero;
use zotero::ZoteroInit;
use zotero::Patch;
use zotero::Get;
use zotero::data_structure::item::ItemType;

let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");

let item = z.get_item("Q8GNE36F", None);

if let Ok(mut result) = item {
    if let ItemType::Book(bookdata) = &mut result.data {
        bookdata.title = "A new title".to_string();
        bookdata.publisher = "A new publisher".to_string();
        z.update_item(&bookdata.key, &bookdata);
    };
    println!("{:?}", serde_json::to_string(&result.data));
};
```
