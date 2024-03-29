# Justfile

# Choose a random port on which to run 'trunk', otherwise concurrent runs
# interfere with each other if they use the same port.
#
trunk_port := `echo $((5170 + $RANDOM % 10))`

export RUSTFLAGS := "--cfg=web_sys_unstable_apis"

cypress_base_url := 'http://localhost:' + trunk_port

default:
  @just --list --justfile {{justfile()}}

set dotenv-load := true

build_npm:
  pnpm install

build: build_npm
  trunk build

clean:
  trunk clean
  rm -fr cypress/screenshots/
  find cypress -name '__diff_output__' -prune -execdir rm -rf {} +
  rm -fr node_modules/
  rm -f pnpm-lock.json

test: lint test-unit test-e2e

test-e2e: build_npm
  CYPRESS_BASE_URL="{{cypress_base_url}}" node ./scripts/wait-on-port.js trunk serve --no-autoreload --port="{{trunk_port}}" -- "{{trunk_port}}" -- npx cypress run -r list -q

test-e2e-local: build_npm
  CYPRESS_BASE_URL="{{cypress_base_url}}" node ./scripts/wait-on-port.js trunk serve --port="{{trunk_port}}" -- "{{trunk_port}}" -- npx cypress open

test-unit:
  cargo nextest run

lint: && audit disallow-unused-cargo-deps
  cargo fmt --all --check
  leptosfmt --check ./src
  cargo clippy --all-targets --all-features -- -D warnings

disallow-unused-cargo-deps:
  cargo machete Cargo.toml

format:
  cargo fmt --all
  leptosfmt ./src

audit:
  cargo audit

dev: build_npm
  trunk serve --port="{{trunk_port}}" --open

publish: 
  trunk build --release --filehash true
  aws cloudfront create-invalidation --distribution-id "$DIST_ID" --paths "/*"
  aws s3 rm "s3://$BUCKET_NAME" --recursive
  aws s3 cp src/dist "s3://$BUCKET_NAME" --recursive
