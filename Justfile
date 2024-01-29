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

clean: kill-server
  rm -rf dist
  trunk clean
  cargo clean
  rm -f assets/css/styles.css
  rm -fr cypress/screenshots/
  rm -fr cypress/snapshots/navigation/desktop-menu-look-and-feel.cy.js/__diff_output__/
  rm -fr cypress/snapshots/navigation/mobile-menu-look-and-feel.cy.js/__diff_output__/
  rm -fr node_modules/
  rm -f package-lock.json

test: test-unit test-e2e

kill-server:
  if [ -e .pid ]; then kill "$(cat .pid)" && rm .pid ; fi

test-e2e: && kill-server
  trunk serve --port=5274 & pid=$!; echo "$pid" > .pid
  sleep 10  # Wait for trunk server to start
  npx cypress run -r list -q
  
test-unit: build
  cargo nextest run

lint: && audit disallow-unused-cargo-deps
  cargo fmt --check
  cargo clippy -- -D warnings
  cargo clippy --all-targets --all-features -- -D warnings

test-ci: lint test-unit test-e2e

disallow-unused-cargo-deps:
  cargo machete Cargo.toml

audit:
  cargo audit

serve: build 
  trunk serve --open --port=$((5170 + $RANDOM % 10))

release: build-release
  trunk build --release --filehash true

pre_build:
  rsync -av assets/* $TRUNK_STAGING_DIR

publish: release
  aws cloudfront create-invalidation --distribution-id "$DIST_ID" --paths "/*"
  aws s3 cp dist "s3://$BUCKET_NAME" --recursive
