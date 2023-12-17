dist_id   := "E3U00NZ5JCWMHS"
bucket_id := "minasearch.com"

default:
  @just --list --justfile {{justfile()}}

build:
  npm install
  cargo build
  trunk build

build-release:
  cargo build --release

clean:
  trunk clean
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

serve: && build
  trunk serve --open

release: && build-release
  trunk build --release --filehash true

publish: release
  aws cloudfront create-invalidation --distribution-id {{dist_id}} --paths "/*"
  aws s3 cp dist s3://{{bucket_id}} --recursive