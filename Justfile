# Justfile
import 'Justfile.dev'

spec := "cypress/e2e/"
trunk_port := `echo $((5170 + $RANDOM % 10))`

export RUSTFLAGS := "--cfg=web_sys_unstable_apis"
export CYPRESS_BASE_URL := 'http://localhost:' + trunk_port
export VERSION := `git rev-parse --short=8 HEAD`
export INDEXER_VERSION := `cd lib/mina-indexer && git rev-parse --short=8 HEAD`
export CARGO_HOME := `pwd` + '/.cargo'

set dotenv-load := true

# Setup block to check and activate Flox if necessary
setup:
    @if [ -z "$FLOX_ENV" ]; then \
        flox activate; \
        if [ $? -ne 0 ]; then \
            echo "Failed to activate Flox environment. Is Flox installed? https://flox.dev/docs/install-flox/"; \
            exit 1; \
        fi; \
    fi

default:
  @echo "Topologically sorted recipes:"
  @just --list --unsorted --list-heading '' --justfile {{justfile()}}

# Deploys the mina-indexer locally
deploy-mina-indexer:
  @echo "--- Deploying mina-indexer at {{INDEXER_VERSION}}"
  ruby ops/validate-env.rb VOLUMES_DIR INDEXER_PORT
  mkdir -p $VOLUMES_DIR/mina-indexer-prod
  cd lib/mina-indexer && VOLUMES_DIR=$VOLUMES_DIR nix develop --command just deploy-local-prod 10000 $INDEXER_PORT

# Shuts down the locally running mina-indexer
shutdown-mina-indexer:
  @echo "--- Shutting down mina-indexer"
  ruby ops/validate-env.rb VOLUMES_DIR
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
  standardrb --fix ops
  cargo-fmt --all
  leptosfmt ./src

# Perform unit tests
test-unit:
  @echo "--- Performing jest unit tests"
  pnpm exec jest test
  @echo "--- Performing ruby tests"
  # test was failing to start/stop process
  # rspec **/*-spec.rb
  @echo "--- Performing rust unit tests"
  cargo-nextest nextest run

# Perform dependency audit
audit:
  cargo-audit audit
  cargo machete

# Install cypress dependencies
pnpm-install:
  @echo "--- Installing NPM dependencies"
  pnpm install

# Serve application on localhost
dev: pnpm-install deploy-mina-indexer
  trap 'just shutdown-mina-indexer' INT; \
  trunk serve --port="{{trunk_port}}" --open


# Run tier2 tests
t2: pnpm-install deploy-mina-indexer && shutdown-mina-indexer
  @echo "--- Performing end-to-end @tier2 tests"
  ruby ops/validate-env.rb GRAPHQL_URL REST_URL
  CYPRESS_tags="@tier2" \
  GRAPHQL_URL="$GRAPHQL_URL" \
  REST_URL="$REST_URL" \
  time ruby ./ops/manage-processes.rb \
    --port={{trunk_port}} \
    --first-cmd="trunk serve --no-autoreload --port={{trunk_port}}" \
    --second-cmd="pnpm exec cypress run -r list -q"

# Run tier2 tests interactively with GUI
t2-i: pnpm-install deploy-mina-indexer
  ruby ops/validate-env.rb GRAPHQL_URL REST_URL
  trap 'just shutdown-mina-indexer' INT; \
  ruby ./ops/manage-processes.rb \
    --port={{trunk_port}} \
    --first-cmd="trunk serve --no-autoreload --port={{trunk_port}}" \
    --second-cmd="pnpm exec cypress open"

# Validates the environment before publishing application to CDN
pre-publish:
  @echo "--- Validating environment variables for publishing"
  ruby ops/validate-env.rb GRAPHQL_URL REST_URL
  ruby -e '["GRAPHQL_URL", "REST_URL"].each { |var| exit 1 if ENV[var].include?("localhost") || ENV[var].include?("127.0.0.1") }; exit 0'

# Publish application
publish: pre-publish clean pnpm-install
  @echo "--- Publishing"
  trunk build --release --filehash true
  @echo "Publishing version {{VERSION}}"
  npx wrangler pages deploy --branch main

# Check rust code
check:
  cargo check

# Lint application source code
lint: pnpm-install && audit
  @echo "--- Linting JS/TS"
  pnpm exec prettier --check cypress/
  @echo "--- Linting ops scripts"
  ruby -cw ops/*.rb
  standardrb --no-fix ops
  @echo "--- Linting Rust code"
  time cargo-fmt --all --check
  leptosfmt --check ./src
  time cargo clippy --all-targets --all-features -- -D warnings

# Run tier1 tests
tier1: lint test-unit

# Run tier2 regression suite in CI
tier2: lint test-unit && t2
