# Justfile
import 'Justfile.dev'

spec := "cypress/e2e/"
trunk_port := `echo $((5170 + $RANDOM % 10))`

export RUSTFLAGS := "--cfg=web_sys_unstable_apis"
export CYPRESS_BASE_URL := 'http://localhost:' + trunk_port
export VERSION := `git rev-parse --short=8 HEAD`
export INDEXER_VERSION := `cd lib/mina-indexer && git rev-parse --short=8 HEAD`
export CARGO_HOME := `pwd` + '/.cargo'
export VOLUMES_DIR := x'${VOLUMES_DIR:-/mnt}'
export INDEXER_PORT := x'${INDEXER_PORT:-8081}'

set dotenv-load := true

default:
  @echo "Topologically sorted recipes:"
  @just --list --unsorted --list-heading '' --justfile {{justfile()}}

deploy-mina-indexer:
  @echo "--- Deploying mina-indexer at {{INDEXER_VERSION}}"
  mkdir -p $VOLUMES_DIR/mina-indexer-prod
  cd lib/mina-indexer && VOLUMES_DIR=$VOLUMES_DIR nix develop --command just deploy-local-prod 10000 {{INDEXER_PORT}}

shutdown-mina-indexer:
  @echo "--- Shutting down mina-indexer"
  $VOLUMES_DIR/mina-indexer-prod/bin/mina-indexer-{{INDEXER_VERSION}} \
    --socket $VOLUMES_DIR/mina-indexer-prod/mina-indexer-{{INDEXER_VERSION}}.sock \
    server \
    shutdown

# Remove build and test artifacts
clean:
  trunk clean
  rm -fr cypress/screenshots/
  find cypress -name '__diff_output__' -prune -execdir rm -rf {} +
  rm -fr node_modules/
  rm -f pnpm-lock.json
  rm -fr cypress/snapshots/
  rm -fr .husky/_
  rm -fr .wrangler
  rm -fr src/dist

# Format rust and cypress source code
format:
  pnpm exec prettier --write cypress/ src/scripts/
  cargo fmt --all
  leptosfmt ./src
  alejandra flake.nix

# Perform unit tests
test-unit:
  @echo "--- Performing unit tests"
  cargo-nextest nextest run

# Perform dependency audit
audit:
  cargo-audit audit
  cargo machete Cargo.toml

# Install cypress dependencies
pnpm_install:
  @echo "--- Installing NPM dependencies"
  pnpm install

# Serve application on localhost
dev: pnpm_install
  trunk serve --port="{{trunk_port}}" --open

# Run tier2 application regression tests
test-e2e-tier2: pnpm_install deploy-mina-indexer && shutdown-mina-indexer
  @echo "--- Performing end-to-end @tier2 tests"
  CYPRESS_tags="@tier2" \
  node ./scripts/wait-on-port.js \
    trunk serve \
    --no-autoreload \
    --port="{{trunk_port}}" \
    -- \
    "{{trunk_port}}" \
    -- \
    pnpm exec cypress run -r list -q

# Run regression tests with interactive GUI
test-e2e-local: pnpm_install deploy-mina-indexer
  node ./scripts/wait-on-port.js \
    trunk serve \
    --no-autoreload \
    --port="{{trunk_port}}" \
    -- \
    "{{trunk_port}}" \
    -- \
    pnpm exec cypress open

# Publish application
publish: clean pnpm_install
  @echo "--- Publishing"
  trunk build --release --filehash true
  @echo "Publishing version {{VERSION}}"
  pnpm exec -- wrangler pages deploy --branch main

# Lint application source code
lint: pnpm_install && audit
  @echo "--- Linting JS/TS"
  pnpm exec prettier --check cypress/
  @echo "--- Linting Rust code"
  cargo fmt --all --check
  leptosfmt --check ./src
  cargo clippy --all-targets --all-features -- -D warnings
  @echo "--- Linting Nix configs"
  alejandra --check flake.nix

# Run tier1 tests
tier1: lint test-unit

# Run tier2 regression suite in CI
tier2: lint test-unit && test-e2e-tier2
