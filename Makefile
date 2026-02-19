BOJ_CONTRACT_TEST ?= 1

.DEFAULT_GOAL := help

.PHONY: help test test-contract test-all build build-release fmt fmt-check clippy doc doc-test doc-check check example-offline example-live

help: ## Show available make targets
	@awk 'BEGIN {FS = ":.*##"; print "Available targets:"} /^[a-zA-Z0-9_-]+:.*##/ {printf "  %-14s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

test: ## Run offline test suite
	cargo test --workspace --all-targets

test-contract: ## Run live BOJ API contract test
	BOJ_CONTRACT_TEST=$(BOJ_CONTRACT_TEST) cargo test --test contract_nightly -- --ignored

test-all: ## Run offline tests, then live contract test
	$(MAKE) test
	$(MAKE) test-contract

build: ## Build workspace in debug mode
	cargo build --workspace --all-targets

build-release: ## Build workspace in release mode
	cargo build --workspace --all-targets --release

fmt: ## Format source files
	cargo fmt --all

fmt-check: ## Check formatting without writing changes
	cargo fmt --all -- --check

clippy: ## Run clippy with warnings denied
	cargo clippy --workspace --all-targets -- -D warnings

doc: ## Build rustdoc with warnings denied
	RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps

doc-test: ## Run doctests
	cargo test --doc --workspace

doc-check: ## Run rustdoc and doctests
	$(MAKE) doc
	$(MAKE) doc-test

check: ## Run fmt-check, clippy, offline tests, and doc checks
	$(MAKE) fmt-check
	$(MAKE) clippy
	$(MAKE) test
	$(MAKE) doc-check

example-offline: ## Run request example with offline fixtures
	cargo run --example request_example -- --mode offline

example-live: ## Run request example against live BOJ API
	cargo run --example request_example -- --mode live
