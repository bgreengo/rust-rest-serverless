export CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
export STACK_NAME=rust-products

all: lint build deploy tests

lint:
	cfn-lint template.yaml
	cargo check
	cargo fmt
	cargo clippy
.PHONY: tests

build:
	cargo build --release --target x86_64-unknown-linux-musl
	sam build
.PHONY: tests

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
	if [ -f samconfig.toml ]; \
		then sam deploy --stack-name $(STACK_NAME); \
		else sam deploy -g --stack-name $(STACK_NAME); \
	fi
.PHONY: tests

tests:
	REST_API=$$(aws cloudformation describe-stacks --stack-name $(STACK_NAME) \
		--query 'Stacks[0].Outputs[?OutputKey==`ApiUrl`].OutputValue' \
		--output text) cargo test
.PHONY: tests
