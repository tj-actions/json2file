.DEFAULT_GOAL := help


.PHONY: help
# Put it first so that "make" without argument is like "make help".
help:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-32s-\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: build
build: clean  ## Build the binary
	@cargo build

.PHONY: test
test:  ## Run the tests
	@cargo test

.PHONY: clean
clean:  ## Clean the build artifacts
	@cargo clean

.PHONY: install
install: build  ## Install the binary
	@cargo install --path .

.PHONY: all
all: build test  ## Build and run the tests