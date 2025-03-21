# Rakefile
require 'open3'
require 'dotenv'  # Add this line to require dotenv

# Load environment variables from .env file
Dotenv.load       # Add this line to load .env file

# Constants and environment variables
SPEC = "cypress/e2e/"
TRUNK_PORT = `echo $((5170 + $RANDOM % 10))`.chomp
ENV['RUSTFLAGS'] = "--cfg=web_sys_unstable_apis"
ENV['CYPRESS_BASE_URL'] = "http://localhost:#{TRUNK_PORT}"
ENV['VERSION'] = `git rev-parse --short=8 HEAD`.chomp
ENV['INDEXER_VERSION'] = `cd lib/mina-indexer && git rev-parse --short=8 HEAD`.chomp
ENV['CARGO_HOME'] = "#{Dir.pwd}/.cargo"

# Helper method for shell commands
def sh(cmd)
  puts cmd
  system(cmd) or raise "Command failed: #{cmd}"
end

# Default task
task :default do
  puts "Available tasks:"
  Rake.application.tasks.each { |t| puts "  #{t.name}" }
end

# Setup task
task :setup do
  unless ENV['FLOX_ENV']
    sh "flox activate"
  end
rescue
  puts "Failed to activate Flox environment. Is Flox installed? https://flox.dev/docs/install-flox/"
  exit 1
end

# Deploy mina-indexer
task :'deploy-mina-indexer' => :setup do
  puts "--- Deploying mina-indexer at #{ENV['INDEXER_VERSION']}"
  sh "mkdir -p $VOLUMES_DIR/mina-indexer-prod"
  Dir.chdir("lib/mina-indexer") do
    sh "VOLUMES_DIR=$VOLUMES_DIR nix develop --command just deploy-local-prod-dev 10000 $INDEXER_PORT"
  end
end

# Shutdown mina-indexer
task :'shutdown-mina-indexer' => :setup do
  puts "--- Shutting down mina-indexer"
  Dir.chdir("lib/mina-indexer") do
    sh "nix develop --command just shutdown prod"
  end
end

# Clean task
task :clean do
  sh "trunk clean"
  sh "rm -fr cypress/screenshots/"
  sh "find cypress -name '__diff_output__' -prune -execdir rm -rf {} +"
  sh "rm -fr node_modules/"
  sh "rm -f pnpm-lock.json"
  sh "rm -fr cypress/snapshots/"
  sh "rm -fr .husky/_"
  sh "rm -fr .wrangler"
  sh "rm -fr src/dist"
end

# Format task
task :format => :'pnpm-install' do
  sh "pnpm exec prettier --write cypress/ src/scripts/"
  sh "standardrb --fix ops"
  sh "cargo-fmt --all"
  sh "leptosfmt ./src"
end

# Test unit
task :'test-unit' do
  puts "--- Performing jest unit tests"
  sh "pnpm exec jest test"
  puts "--- Performing rust unit tests"
  sh "cargo-nextest nextest run"
end

# Audit task
task :audit do
  sh "cargo-audit audit"
  sh "cargo machete"
end

# Fix linting errors
task :'lint-fix' do
  sh "standardrb --fix ops/*.rb"
  sh "cargo clippy --fix --allow-dirty --allow-staged"
end

# Builds documentation in the home directory
task :'build-docs' do
  sh "rm -rf $HOME/mina_block_explorer_docs/"
  sh "cargo doc --document-private-items --target-dir $HOME/mina_block_explorer_docs/"
end

# PNPM install
task :'pnpm-install' do
  puts "--- Installing NPM dependencies"
  sh "pnpm install"
end

# Dev task
task :dev => [:'pnpm-install', :'deploy-mina-indexer'] do
  trap("INT") { sh "just shutdown-mina-indexer" }
  sh "trunk serve --port=#{TRUNK_PORT} --open"
end

# Tier2 tests
task :t2 => [:'pnpm-install', :'deploy-mina-indexer'] do
  puts "--- Performing end-to-end @tier2 tests"
  sh "ruby ops/validate-env.rb GRAPHQL_URL REST_URL"
  sh "CYPRESS_tags='@tier2' GRAPHQL_URL='$GRAPHQL_URL' REST_URL='$REST_URL' time ruby ./ops/manage-processes.rb --port=#{TRUNK_PORT} --first-cmd='trunk serve --no-autoreload --port=#{TRUNK_PORT}' --second-cmd='pnpm exec cypress run -r list -q'"
end

# Interactive Tier2 tests
task :'t2-i' => [:'pnpm-install', :'deploy-mina-indexer'] do
  sh "ruby ops/validate-env.rb GRAPHQL_URL REST_URL"
  trap("INT") { sh "just shutdown-mina-indexer" }
  sh "ruby ./ops/manage-processes.rb --port=#{TRUNK_PORT} --first-cmd='trunk serve --no-autoreload --port=#{TRUNK_PORT}' --second-cmd='pnpm exec cypress open'"
end

# Pre-publish validation
task :'pre-publish' do
  puts "--- Validating environment variables for publishing"
  sh "ruby ops/validate-env.rb GRAPHQL_URL REST_URL"
  sh "ruby -e 'exit ![\"GRAPHQL_URL\", \"REST_URL\"].any? { |var| [\"localhost\", \"127.0.0.1\"].any? { |str| ENV[var]&.include?(str) } }'"
end

# Publish task
task :publish => [:'pre-publish', :clean, :'pnpm-install'] do
  puts "--- Publishing"
  sh "trunk build --release --filehash true"
  puts "Publishing version #{ENV['VERSION']}"
  sh "pnpx wrangler pages deploy --branch main"
end

# Check task
task :check do
  sh "cargo check"
end

# Lint task
task :lint => [:'pnpm-install', :audit] do
  puts "--- Linting JS/TS"
  sh "pnpm exec prettier --check cypress/"
  puts "--- Linting ops scripts"
  sh "ruby -cw ops/*.rb"
  sh "standardrb --no-fix ops"
  puts "--- Linting Rust code"
  sh "time cargo-fmt --all --check"
  sh "leptosfmt --check ./src"
  sh "time cargo clippy --all-targets --all-features -- -D warnings"
end

# Tier1 tests
task :tier1 => [:lint, :'test-unit']

# Tier2 regression suite
task :tier2 => [:lint, :'test-unit', :t2]
