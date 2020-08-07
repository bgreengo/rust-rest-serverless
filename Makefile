export CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc

all: lint build deploy

lint:
	cfn-lint template.yaml
	cargo check
	cargo fmt
	cargo clippy

build:
	cargo build --release --target x86_64-unknown-linux-musl
	sam build

build-CreateProductFunction:
	cp ./target/x86_64-unknown-linux-musl/release/create-product $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap

build-DeleteProductFunction:
	cp ./target/x86_64-unknown-linux-musl/release/delete-product $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap

build-GetProductFunction:
	cp ./target/x86_64-unknown-linux-musl/release/get-product $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap

build-GetProductsFunction:
	cp ./target/x86_64-unknown-linux-musl/release/get-products $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap

deploy:
	if [ -f samconfig.toml ]; then sam deploy; else sam deploy -g; fi
