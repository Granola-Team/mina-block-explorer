# Justfile

# Choose a random port on which to run 'trunk', otherwise concurrent runs
# interfere with each other if they use the same port.
#
trunk_port := `echo $((5170 + $RANDOM % 10))`

export RUSTFLAGS := "--cfg=web_sys_unstable_apis"

export CYPRESS_BASE_URL := 'http://localhost:' + trunk_port

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
  rm -fr cypress/snapshots/
  rm -fr .husky/_
  rm -fr src/dist

test: lint test-unit test-e2e

test-e2e: build_npm
  node ./scripts/wait-on-port.js \
    trunk serve \
    --no-autoreload \
    --port="{{trunk_port}}" \
    -- \
    "{{trunk_port}}" \
    -- \
    pnpm exec cypress run -r list -q

test-e2e-local: build_npm
  node ./scripts/wait-on-port.js \
    trunk serve \
    --no-autoreload \
    --port="{{trunk_port}}" \
    -- \
    "{{trunk_port}}" \
    -- \
    pnpm exec cypress open

test-unit:
  cargo nextest run

lint: build && audit disallow-unused-cargo-deps
  pnpm exec prettier --check cypress/
  cargo fmt --all --check
  leptosfmt --check ./src
  cargo clippy --all-targets --all-features -- -D warnings

disallow-unused-cargo-deps:
  cargo machete Cargo.toml

format:
  pnpm exec prettier --write cypress/ src/scripts/
  cargo fmt --all
  leptosfmt ./src

audit:
  cargo audit

dev: build_npm
  trunk serve --port="{{trunk_port}}" --open

publish: clean build_npm
  trunk build --release --filehash true
  pnpm exec -- wrangler pages deploy --project-name minasearch --branch production
