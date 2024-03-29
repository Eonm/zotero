use std::env;
use std::fs::File;
use std::io::Read;

use zotero_data::item::Item;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1]).unwrap();
    let mut data = String::new();
    let _ = f.read_to_string(&mut data);

    let items: Vec<Item> = serde_json::from_str(&data).unwrap();
    println!("{items:?}");
}
