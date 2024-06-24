# Justfile

# Choose a random port on which to run 'trunk', otherwise concurrent runs
# interfere with each other if they use the same port.
#

import 'Justfile.dev'

spec := "cypress/e2e/"
trunk_port := `echo $((5170 + $RANDOM % 10))`

export RUSTFLAGS := "--cfg=web_sys_unstable_apis"

export CYPRESS_BASE_URL := 'http://localhost:' + trunk_port

export VERSION := `git rev-parse --short=8 HEAD`

export CARGO_HOME := `pwd` + '.cargo'
export BERKELEY_FEATURES_ENABLED := "true"

set dotenv-load := true

default:
  @echo "Topologically sorted recipes:"
  @just --list --unsorted --list-heading '' --justfile {{justfile()}}

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

format:
  pnpm exec prettier --write cypress/ src/scripts/
  cargo fmt --all
  leptosfmt ./src

test-unit:
  @echo "--- Performing unit tests"
  cargo-nextest nextest run

audit:
  cargo-audit audit

disallow-unused-cargo-deps:
  cargo machete Cargo.toml

pnpm_install:
  @echo "--- Installing NPM dependencies"
  pnpm install

dev: pnpm_install
  trunk serve --port="{{trunk_port}}" --open

test-e2e: pnpm_install
  @echo "--- Performing end-to-end tests"
  CYPRESS_tags='' \
  node ./scripts/wait-on-port.js \
    trunk serve \
    --no-autoreload \
    --port="{{trunk_port}}" \
    -- \
    "{{trunk_port}}" \
    -- \
    pnpm exec cypress run -r list -q

test-e2e-tier1 spec=spec: pnpm_install
  @echo "--- Performing end-to-end @tier1 tests"
  CYPRESS_tags="@tier1" \
  node ./scripts/wait-on-port.js \
    trunk serve \
    --no-autoreload \
    --port="{{trunk_port}}" \
    -- \
    "{{trunk_port}}" \
    -- \
    pnpm exec cypress run -r list -q --spec {{spec}}

test-e2e-tier2: pnpm_install
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

test-e2e-local: pnpm_install
  node ./scripts/wait-on-port.js \
    trunk serve \
    --no-autoreload \
    --port="{{trunk_port}}" \
    -- \
    "{{trunk_port}}" \
    -- \
    pnpm exec cypress open

publish: clean pnpm_install
  @echo "--- Publishing"
  trunk build --release --filehash true
  @echo "Publishing version {{VERSION}}"
  pnpm exec -- wrangler pages deploy --branch main

lint: pnpm_install && audit disallow-unused-cargo-deps
  @echo "--- Linting"
  pnpm exec prettier --check cypress/
  cargo fmt --all --check
  leptosfmt --check ./src
  cargo clippy --all-targets --all-features -- -D warnings

tier1: lint test-unit && (test-e2e-tier1 spec)

tier2: lint test-unit && test-e2e-tier2
