default:
  @just --list --justfile {{justfile()}}

build:
  cargo build

clean:
  cargo clean

test: test-unit

test-e2e:
  echo "Testing E2E"
  # npm install
  # npx playwright install --with-deps
  # npx playwright test

test-unit: build
  cargo nextest run

lint: && audit disallow-unused-cargo-deps
  cargo clippy -- -D warnings
  cargo clippy --all-targets --all-features -- -D warnings
  cargo check

test-ci: lint test-unit test-e2e

disallow-unused-cargo-deps:
  cargo machete Cargo.toml

audit:
  cargo audit
