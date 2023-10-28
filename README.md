<div class="center" align="center">
    
# Zotero

[![Build Status](https://travis-ci.org/Eonm/zotero.svg?branch=master)](https://travis-ci.org/Eonm/zotero)
[![Coverage Status](https://coveralls.io/repos/github/Eonm/zotero/badge.svg?branch=master)](https://coveralls.io/github/Eonm/zotero?branch=master)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/Eonm/zotero/issues)
[![API](https://docs.rs/zotero/badge.svg)](https://docs.rs/zotero)
[![dependency status](https://deps.rs/repo/github/eonm/zotero/status.svg)](https://deps.rs/repo/github/eonm/zotero)

</div>

[API documentation](https://docs.rs/zotero/)

## Creating items and collections

``` rust
use zotero_api::{Zotero, ZoteroApi, ZoteroApiExecutor};
use zotero_data::item::{BookData, BookDataBuilder, Creator, CreatorBuilder};
let z = Zotero::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
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
let _: Result<(), _> = z.create_new_item(new_book).execute(&z);
```

## Updating items and collections

``` rust
use zotero_api::{Zotero, ZoteroApi, ZoteroApiExecutor};
use zotero_data::item::ItemType;
let z = Zotero::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
let item: Result<ItemType, _> = z.get_item("Q8GNE36F", None).execute(&z);
if let Ok(mut result) = item {
    if let ItemType::Book(bookdata) = &mut result {
        bookdata.title = "A new title".to_string();
        bookdata.publisher = "A new publisher".to_string();
        let _: Result<(),_> =z.update_item(&bookdata.key, &bookdata).execute(&z);
    };

    println!("{:?}", serde_json::to_string(&result));
};
```

## Async Support
``` rust
use zotero_api::{Zotero, ZoteroApi, ZoteroApiAsyncExecutor};
use zotero_data::Item;

let z = Zotero::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
let item_result: Result<Item, _> = z.get_item("Q8GNE36F", None).execute(&z).await;
println!("{item_result:#?}")
```
