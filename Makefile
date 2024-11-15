run:
	cargo run

build:
	cargo build

lint:
	rustup run nightly cargo fmt --check

log:
	git cliff -o CHANGELOG.md
	