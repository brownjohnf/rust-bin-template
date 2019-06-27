.PHONY: name build test release install readme doc clean
.DEFAULT_GOAL := build

name:
	echo $(binname)

build:
	cargo build

test: build
	cargo test

release: test
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
