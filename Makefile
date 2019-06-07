.PHONY: name build test release install readme doc clean
.DEFAULT_GOAL := build

binname := $(shell cat Cargo.toml | awk '/name =/ {gsub("\"", "", $$3); print $$3}')

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
	sudo cp target/release/$(binname) \
		/usr/local/bin/$(binname)

clean:
	rm -rf \
		target \
		dist
