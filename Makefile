RUSTV=stable

build:
	cargo build --all-features
	cargo test --all 

install-fmt:
	rustup component add rustfmt --toolchain $(RUSTV)

check-fmt:
	cargo +$(RUSTV) fmt -- --check
	

install-clippy:
	rustup component add clippy --toolchain $(RUSTV)

check-clippy:	install-clippy
	cargo +$(RUSTV) clippy --all-targets  -- -D warnings
	cd src/client; cargo +$(RUSTV) clippy --all-targets  -- -D warnings
