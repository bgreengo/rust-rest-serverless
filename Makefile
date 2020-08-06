export CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc

build:
	cargo build --release --target x86_64-unknown-linux-musl
	sam build

build-GetProductFunction:
	cp ./target/x86_64-unknown-linux-musl/release/get-product $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap

build-GetProductsFunction:
	cp ./target/x86_64-unknown-linux-musl/release/get-products $(ARTIFACTS_DIR)/bootstrap
	strip $(ARTIFACTS_DIR)/bootstrap
