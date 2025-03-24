# Rakefile
require "open3"
require "dotenv"  # Add this line to require dotenv
require "fileutils"

# Load environment variables from .env file
Dotenv.load       # Add this line to load .env file

# Constants and environment variables
SPEC = "cypress/e2e/"
TRUNK_PORT = `echo $((5170 + $RANDOM % 10))`.chomp
ENV["RUSTFLAGS"] = "--cfg=web_sys_unstable_apis"
ENV["CYPRESS_BASE_URL"] = "http://localhost:#{TRUNK_PORT}"
ENV["VERSION"] = `git rev-parse --short=8 HEAD`.chomp
ENV["INDEXER_VERSION"] = `cd lib/mina-indexer && git rev-parse --short=8 HEAD`.chomp
ENV["CARGO_HOME"] = "#{Dir.pwd}/.cargo"
RUST_SRC_FILES = Dir.glob("src/**/*.rs") + Dir.glob("graphql/**/*.graphql")
CARGO_DEPS = RUST_SRC_FILES + ["Cargo.toml", "Cargo.lock", "build.rs"]
CYPRESS_FILES = Dir.glob("cypress/**/*.js")
RUBY_SRC_FILES = Dir.glob("**/*.rb").reject { |file| file.start_with?("lib/") } + ["Rakefile"]
JAVASCRIPT_SRC_FILES = Dir.glob("src/scripts_tests/**")

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

# Deploy mina-indexer
task :deploy_mina_indexer do
  puts "--- Deploying mina-indexer at #{ENV["INDEXER_VERSION"]}"
  sh "mkdir -p $VOLUMES_DIR/mina-indexer-prod"
  Dir.chdir("lib/mina-indexer") do
    sh "VOLUMES_DIR=$VOLUMES_DIR nix develop --command just deploy-local-prod-dev 10000 $INDEXER_PORT"
  end
end

file ".build" do |t|
  mkdir_p t.name # Creates the 'build' directory if it doesn’t exist
end

# Shutdown mina-indexer
task :shutdown_mina_indexer do
  puts "--- Shutting down mina-indexer"
  Dir.chdir("lib/mina-indexer") do
    sh "nix develop --command just shutdown prod"
  end
end

# Clean task
task :clean do
  sh "trunk clean"

  FileUtils.rm_rf %w[
    cypress/screenshots
    node_modules
    .husky/_
    .wrangler
    src/dist
    .build
  ]
end

# Format task
task format: :pnpm_install do
  sh "pnpm exec prettier --write cypress/ src/scripts/"
  sh "standardrb --fix ops/*.rb Rakefile"
  sh "cargo-fmt --all"
  sh "leptosfmt ./src"
end

task jest_test: ".build/jest-test"
file ".build/jest-test" => JAVASCRIPT_SRC_FILES + [".build", "jest.config.js"] do |t|
  puts "--- Performing jest unit tests"
  sh "pnpm exec jest test 2>&1 | tee #{t.name}"
end

task rust_test: ".build/rust-test"
file ".build/rust-test" => CARGO_DEPS + [".build"] do |t|
  puts "--- Performing rust unit tests"
  sh "cargo-nextest nextest run 2>&1 | tee #{t.name}"
end

# Test unit
task test_unit: [:jest_test, :rust_test]

# Audit taskd
task audit: ".build/audit"
file ".build/audit" => CARGO_DEPS + [".build"] do |t|
  audit_output = `cargo-audit audit`
  machete_output = `cargo machete`
  File.write(t.name, [audit_output, machete_output].join("\n"))
end

# Fix linting errors
task :lint_fix do
  sh "standardrb --fix ops/*.rb Rakefile"
  sh "cargo clippy --fix --allow-dirty --allow-staged"
end

# Builds documentation in the home directory
task :build_docs do
  sh "rm -rf $HOME/mina_block_explorer_docs/"
  sh "cargo doc --document-private-items --target-dir $HOME/mina_block_explorer_docs/"
end

# PNPM install
task pnpm_install: "node_modules"
file "node_modules" => ["pnpm-lock.yaml", "package.json"] do
  puts "--- Installing NPM dependencies"
  sh "pnpm install"
end

# Dev task
task dev: [:pnpm_install, :deploy_mina_indexer] do
  trap("INT") { shutdown_mina_indexer }
  sh "trunk serve --port=#{TRUNK_PORT} --open"
end

# Tier2 tests
task t2: [:pnpm_install, :build, :deploy_mina_indexer] do
  puts "--- Performing end-to-end @tier2 tests"
  raise "Error: Neither GRAPHQL_URL nor REST_URL contains 'localhost' or '127.0.0.1'" unless
    ["GRAPHQL_URL", "REST_URL"].any? { |var| ["localhost", "127.0.0.1"].any? { |str| ENV[var]&.include?(str) } }
  sh "CYPRESS_tags='@tier2' GRAPHQL_URL=#{ENV["GRAPHQL_URL"]} REST_URL=#{ENV["REST_URL"]} time ruby ./ops/manage-processes.rb --port=#{TRUNK_PORT} --first-cmd='trunk serve --no-autoreload --port=#{TRUNK_PORT}' --second-cmd='pnpm exec cypress run -r list -q'"
end

# Interactive Tier2 tests
task t2_i: [:pnpm_install, :build, :deploy_mina_indexer] do
  raise "Error: Neither GRAPHQL_URL nor REST_URL contains 'localhost' or '127.0.0.1'" unless
    ["GRAPHQL_URL", "REST_URL"].any? { |var| ["localhost", "127.0.0.1"].any? { |str| ENV[var]&.include?(str) } }
  trap("INT") { shutdown_mina_indexer }
  sh "ruby ./ops/manage-processes.rb --port=#{TRUNK_PORT} --first-cmd='trunk serve --no-autoreload --port=#{TRUNK_PORT}' --second-cmd='pnpm exec cypress open'"
end

# Pre-publish validation
task :pre_publish do
  puts "--- Validating environment variables for publishing"
  sh "ruby ops/validate-env.rb GRAPHQL_URL REST_URL"
  raise "GRAPHQL_URL or REST_URL contains 'localhost' or '127.0.0.1'" if
    ["GRAPHQL_URL", "REST_URL"].any? { |var| ["localhost", "127.0.0.1"].any? { |str| ENV[var]&.include?(str) } }
end

# Publish task
task publish: [:tier1, :pre_publish, :clean, :pnpm_install, :build] do
  puts "--- Publishing"
  puts "Publishing version #{ENV["VERSION"]}"
  sh "pnpx wrangler pages deploy --branch main"
end

# Check task
task check: ".build/check"

file ".build/check" => CARGO_DEPS + [".build"] do |t|
  sh "cargo check 2>&1 | tee #{t.name}"
end

task lint_javascript: ".build/lint-javascript"
file ".build/lint-javascript" => CYPRESS_FILES + [".build"] do |t|
  puts "--- Linting JS/TS"
  sh "pnpm exec prettier --check cypress/ 2>&1 | tee #{t.name}"
end

task lint_ruby: ".build/lint-ruby"
file ".build/lint-ruby" => RUBY_SRC_FILES + [".build"] do |t|
  puts "--- Linting ruby scripts"
  ruby_cw_output = `ruby -cw #{RUBY_SRC_FILES.join(" ")}`
  ruby_std_output = `standardrb --no-fix #{RUBY_SRC_FILES.join(" ")} Rakefile`
  File.write(t.name, [ruby_cw_output, ruby_std_output].join("\n"))
end

task lint_rust: ".build/lint-rust"
file ".build/lint-rust" => RUST_SRC_FILES + [".build", "rustfmt.toml"] do |t|
  puts "--- Linting Rust code"
  cargo_fmt_out = `cargo-fmt --all --check`
  leptos_fmt_out = `leptosfmt --check ./src`
  clippy_out = `cargo clippy --all-targets --all-features -- -D warnings`
  File.write(t.name, [cargo_fmt_out, leptos_fmt_out, clippy_out].join("\n"))
end

task build: 'dist'
file 'dist' => CARGO_DEPS + ['Trunk.toml', 'tailwind.config.js'] do
  sh "trunk build --release --filehash true"
end

# Lint task
task lint: [:pnpm_install, :audit, :lint_javascript, :lint_ruby, :lint_rust]

# Tier1 tests
task tier1: [:lint, :test_unit, :build]

# Tier2 regression suite
task tier2: [:tier1, :t2]
