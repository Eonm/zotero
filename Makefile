test:
	cargo test

build-doc:
	cargo doc

open-doc:
	cargo doc --open

serve-doc:
	cargo watch -s 'cargo doc --no-deps && browser-sync start --ss target/doc/zotero -s target/doc --directory --no-open'

coverage: clean
	cargo kcov --lib -- --exclude-path=src/data_structure/item/item_data --exclude-pattern=/.cargo

open-coverage: ./target/cov/index.html
	xdg-open ./target/cov/index.html

clean:
	cargo clean