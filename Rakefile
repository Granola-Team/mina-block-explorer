# Rakefile
require "open3"
require "fileutils"

# Constants and environment variables
SPEC = "cypress/e2e/"
TRUNK_PORT = rand(5170..5179).to_s
ENV["RUSTFLAGS"] = "--cfg=web_sys_unstable_apis"
ENV["CYPRESS_BASE_URL"] = "http://localhost:#{TRUNK_PORT}"
ENV["VERSION"] = `git rev-parse --short=8 HEAD`.chomp
ENV["INDEXER_VERSION"] = `cd lib/mina-indexer && git rev-parse --short=8 HEAD`.chomp
ENV["CARGO_HOME"] = "#{Dir.pwd}/.cargo"
GRAPHQL_SRC_FILES = Dir.glob("graphql/**/*.graphql")
RUST_SRC_FILES = Dir.glob("src/**/*.rs") + GRAPHQL_SRC_FILES
CARGO_DEPS = RUST_SRC_FILES + ["Cargo.toml", "Cargo.lock", "build.rs"]
CYPRESS_FILES = Dir.glob("cypress/**/*.js")
RUBY_SRC_FILES = Dir.glob("**/*.rb").reject { |file| file.start_with?("lib/") } + ["Rakefile"]
JAVASCRIPT_SRC_FILES = Dir.glob("src/scripts_tests/**")

def ensure_env_vars(required_vars, error_context = "Task failed")
  missing_vars = required_vars.reject { |var| ENV[var] && !ENV[var].empty? }
  unless missing_vars.empty?
    abort(
      "Error: #{error_context}. The following required environment variables are missing or empty:\n" +
      missing_vars.map { |var| "  - #{var}" }.join("\n") +
      "\nPlease set these variables and try again."
    )
  end
end

# Helper method for shell commands
def sh(cmd)
  puts cmd
  system(cmd) or raise "Command failed: #{cmd}"
end

desc "Default task - print the menu of targets"
task :default do
  system("rake -T")
end

desc "Deploy mina-indexer"
task :deploy_mina_indexer do
  ensure_env_vars(%w[VOLUMES_DIR], "Cannot deploy mina indexer")
  puts "--- Deploying mina-indexer at #{ENV["INDEXER_VERSION"]}"
  Dir.chdir("lib/mina-indexer") do
    sh %W[
      GRAPHQL_URL=http://localhost:8080/graphql
      REST_URL=http://localhost:8080
      nix develop --command just deploy-local-prod-dev 10000 8080
    ].join(" ")
  end
end

file ".build" do |t|
  mkdir_p t.name # Creates the 'build' directory if it doesn’t exist
end

desc "Shut down mina-indexer"
task :shutdown_mina_indexer do
  puts "--- Shutting down mina-indexer"
  Dir.chdir("lib/mina-indexer") do
    sh "nix develop --command just shutdown prod"
  end
end

desc "Clean the repo of built artifacts"
task :clean do
  puts "--- Cleaning"
  sh "trunk clean"

  FileUtils.rm_rf %w[
    cypress/screenshots
    node_modules
    .wrangler
    src/dist
    .build
  ]
end

desc "Format the source code"
task format: [:pnpm_install] do
  sh "pnpm exec prettier --write cypress/ src/scripts/"
  sh "standardrb --fix ops/*.rb Rakefile"
  sh "cargo-fmt --all"
  sh "leptosfmt ./src"
end

desc "Run the Jest tests"
task jest_test: ".build/jest-test"
file ".build/jest-test" => JAVASCRIPT_SRC_FILES + [".build", "jest.config.js"] do |t|
  puts "--- Performing jest unit tests"
  sh "pnpm exec jest test 2>&1 | tee #{t.name}"
end

desc "Test the Rust code"
task rust_test: ".build/rust-test"

file ".build/rust-test" => CARGO_DEPS + [".build"] do |t|
  puts "--- Performing rust unit tests"
  sh "cargo-nextest nextest run 2>&1 | tee #{t.name}"
end

desc "Run the unit tests"
task test_unit: [:jest_test, :rust_test]

desc "Audit the Rust code with cargo-audit"
task audit: ".build/audit"

file ".build/audit" => CARGO_DEPS + [".build"] do |t|
  audit_output = `cargo-audit audit`
  machete_output = `cargo machete`
  File.write(t.name, [audit_output, machete_output].join("\n"))
end

desc "Fix linting errors"
task :lint_fix do
  sh "standardrb --fix ops/*.rb Rakefile"
  sh "cargo clippy --fix --allow-dirty --allow-staged"
end

desc "Build documentation in the build directory"
task build_docs: ".build/docs"

file ".build/docs" => GRAPHQL_SRC_FILES + [".build"] do |t|
  sh "cargo doc --document-private-items --target-dir #{t.name}"
end

desc "Install the JavaScript dependencies with 'pnpm'"
task pnpm_install: "node_modules"
file "node_modules" => ["pnpm-lock.yaml", "package.json"] do
  puts "--- Installing NPM dependencies"
  sh "pnpm install"
end

desc "Serve the built website locally"
task dev: [:pnpm_install, :deploy_mina_indexer] do
  ensure_env_vars(%w[GRAPHQL_URL REST_URL], "Cannot start dev")
  trap("INT") { Rake::Task["shutdown_mina_indexer"].invoke }
  sh "trunk serve --port=#{TRUNK_PORT} --open"
end

desc "Invoke the interactive Tier2 tests"
task t2_i: [:pnpm_install, :dev_build, :deploy_mina_indexer] do
  trap("INT") { Rake::Task["shutdown_mina_indexer"].invoke }
  sh %W[
    GRAPHQL_URL=http://localhost:8080/graphql
    REST_URL=http://localhost:8080
    ruby ./ops/manage-processes.rb
    --port=#{TRUNK_PORT}
    --first-cmd='trunk serve
    --no-autoreload
    --port=#{TRUNK_PORT}'
    --second-cmd='pnpm exec cypress open'
  ].join(" ")
end

desc "Publish the website to production"
task publish: [:clean, :pnpm_install, :release_build] do
  ensure_env_vars(%w[CLOUDFLARE_ACCOUNT_ID CLOUDFLARE_API_TOKEN], "Cannot publish")

  puts "--- Publishing"
  puts "Publishing version #{ENV["VERSION"]}"
  sh "pnpx wrangler pages deploy --branch main"
end

desc "Use 'cargo check' to verify buildability"
task check: ".build/check"

file ".build/check" => CARGO_DEPS + [".build"] do |t|
  sh "cargo check 2>&1 | tee #{t.name}"
end

desc "Lint the Cypress test code (JavaScript)"
task lint_javascript: ".build/lint-javascript"

file ".build/lint-javascript" => CYPRESS_FILES + [".build"] do |t|
  puts "--- Linting JS/TS"
  sh "pnpm exec prettier --check cypress/ 2>&1 | tee #{t.name}"
end

desc "Lint the Ruby code"
task lint_ruby: ".build/lint-ruby"

file ".build/lint-ruby" => RUBY_SRC_FILES + [".build"] do |t|
  puts "--- Linting ruby scripts"
  ruby_cw_output = `ruby -cw #{RUBY_SRC_FILES.join(" ")}`
  ruby_std_output = `standardrb --no-fix #{RUBY_SRC_FILES.join(" ")} Rakefile`
  File.write(t.name, [ruby_cw_output, ruby_std_output].join("\n"))
end

desc "Lint the Rust code"
task lint_rust: ".build/lint-rust"

file ".build/lint-rust" => RUST_SRC_FILES + [".build", "rustfmt.toml"] do |t|
  puts "--- Linting Rust code"
  cargo_fmt_out = `cargo-fmt --all --check`
  leptos_fmt_out = `leptosfmt --check ./src`
  clippy_out = `cargo clippy --all-targets --all-features -- -D warnings`
  File.write(t.name, [cargo_fmt_out, leptos_fmt_out, clippy_out].join("\n"))
end

desc "Build the dev version for front-end WASM bundle"
task :dev_build do
  ENV['GRAPHQL_URL'] = "http://localhost:8080/graphql"
  ENV['REST_URL'] = "https://localhost:8080"
  Rake::Task["dist"].invoke
end

desc "Build the release version for front-end WASM bundle"
task :release_build do
  ENV['GRAPHQL_URL'] = "https://api.minasearch.com/graphql"
  ENV['REST_URL'] = "https://api.minasearch.com"
  Rake::Task["dist"].invoke
end

file "dist" => CARGO_DEPS + ["Trunk.toml", "tailwind.config.js"] do
  ensure_env_vars(%w[GRAPHQL_URL REST_URL], "Cannot build dist folder")
  sh "trunk build --release --filehash true"
end

desc "Lint all source code"
task lint: [:pnpm_install, :audit, :lint_javascript, :lint_ruby, :lint_rust]

desc "Run the Tier1 tests"
task tier1: [:lint, :test_unit, :dev_build]

desc "Invoke the Tier2 regression suite"
task tier2: [:tier1, :pnpm_install, :deploy_mina_indexer] do
  puts "--- Performing end-to-end @tier2 tests"
  sh %W[
    CYPRESS_tags=@tier2
    GRAPHQL_URL=http://localhost:8080/graphql
    REST_URL=http://localhost:8080
    time ruby ./ops/manage-processes.rb
    --port=#{TRUNK_PORT}
    --first-cmd='trunk serve
    --no-autoreload
    --port=#{TRUNK_PORT}'
    --second-cmd='pnpm exec cypress run -r list -q'
  ].join(" ")
end
