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

tailwind:
  npx tailwindcss -i assets/css/input.css -o assets/css/styles.css --minify

tailwind-watch:
  npx tailwindcss -i assets/css/input.css -o assets/css/styles.css --watch
  
build:
  npm install
  just tailwind
  cargo build --target=wasm32-unknown-unknown
  trunk build

build-release:
  npm install
  just tailwind
  cargo build --release

clean:
  rm -rf dist
  trunk clean
  cargo clean
  rm -f assets/css/styles.css
  rm -fr cypress/screenshots/
  rm -fr cypress/snapshots/navigation/desktop-menu-look-and-feel.cy.js/__diff_output__/
  rm -fr cypress/snapshots/navigation/mobile-menu-look-and-feel.cy.js/__diff_output__/
  rm -fr node_modules/
  rm -f package-lock.json

test: lint test-unit test-e2e

test-e2e: 
  CYPRESS_BASE_URL="{{cypress_base_url}}" node ./scripts/wait-on-port.js trunk serve --port="{{trunk_port}}" -- "{{trunk_port}}" -- npx cypress run -r list -q
  
test-unit: build
  cargo nextest run

lint: && audit disallow-unused-cargo-deps
  cargo fmt --check
  leptosfmt --check ./src
  cargo clippy -- -D warnings
  cargo clippy --all-targets --all-features -- -D warnings

disallow-unused-cargo-deps:
  cargo machete Cargo.toml

format:
  cargo fmt
  leptosfmt ./src

audit:
  cargo audit

dev: build 
  trunk serve --port="{{trunk_port}}" --open

release: build-release
  trunk build --release --filehash true

pre_build:
  rsync -av assets/* $TRUNK_STAGING_DIR

publish: release
  aws cloudfront create-invalidation --distribution-id "$DIST_ID" --paths "/*"
  aws s3 cp dist "s3://$BUCKET_NAME" --recursive
