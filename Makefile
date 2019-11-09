.PHONY: build test release install readme doc clean version
.DEFAULT_GOAL := build

build:
	cargo build

test:
	cargo test --nocapture

release:
	cargo build --release

readme:
	cargo run -- --help > README.txt

doc:
	cargo doc

install: release
	for f in src/bin/*.rs; do \
		sudo cp target/release/$$(basename $${f%.*}) \
			/usr/local/bin/; \
	done

clean:
	rm -rf \
		target \
		dist

version: readme
	verto
