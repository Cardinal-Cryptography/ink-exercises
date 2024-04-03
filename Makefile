.PHONY: help
help: ## Displays this help
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make \033[1;36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[1;36m%-25s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: build-contracts
build-contracts: ## Build the contracts
	@cargo contract build --release --manifest-path contracts/common-staking/Cargo.toml
	@cargo contract build --release --manifest-path contracts/enroll/Cargo.toml
	@cargo contract build --release --manifest-path contracts/voting/Cargo.toml
	@cargo contract build --release --manifest-path contracts/weighted-voting/Cargo.toml

.PHONY: check-exercises
check-exercises: ## Run the exercises
	@cargo test --manifest-path exercises/1-drink-test/tests/Cargo.toml --release
	@cargo test --manifest-path exercises/2-runtime-call/tests/Cargo.toml --release
	@cargo test --manifest-path exercises/3-chain-extension/tests/Cargo.toml --release

.PHONY: all
all: build-contracts check-exercises ## Run all the targets
