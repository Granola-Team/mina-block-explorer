require "open3"
require "fileutils"
require "socket"

# Constants and environment variables
SPEC = "cypress/e2e/"
TRUNK_PORT = rand(5170..5179).to_s
ENV["RUSTFLAGS"] = "--cfg=web_sys_unstable_apis"
ENV["CYPRESS_BASE_URL"] = "http://localhost:#{TRUNK_PORT}"
ENV["VERSION"] = `git rev-parse --short=8 HEAD`.chomp
ENV["INDEXER_VERSION"] = `cd lib/mina-indexer && git rev-parse --short=8 HEAD`.chomp
ENV["CARGO_HOME"] = "#{Dir.pwd}/.cargo"
ENV["VOLUMES_DIR"] ||= "/mnt" if Dir.exist?("/mnt")
GRAPHQL_SRC_FILES = Dir.glob("graphql/**/*.graphql")
RUST_SRC_FILES = Dir.glob("src/**/*.rs") + GRAPHQL_SRC_FILES
CARGO_DEPS = RUST_SRC_FILES + ["Cargo.toml", "Cargo.lock", "build.rs"]
CYPRESS_FILES = Dir.glob("cypress/**/*.js")
RUBY_SRC_FILES = Dir.glob("**/*.rb").reject { |file| file.start_with?("lib/") } + ["Rakefile"]
JAVASCRIPT_SRC_FILES = Dir.glob("src/scripts_tests/**")
MINASEARCH_GRAPHQL = "https://api.minasearch.com/graphql"
MINASEARCH_REST = "https://api.minasearch.com"

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

def sh(cmd)
  puts cmd
  system(cmd) or raise "Command failed: #{cmd}"
end

def record_output(task, outputs)
  outputs = outputs.is_a?(String) ? [outputs] : outputs
  FileUtils.mkdir_p(".build")
  File.write(task.name, outputs.join("\n"))
end

# Check if port is open
def port_open?(port, host = "127.0.0.1")
  Socket.tcp(host, port, connect_timeout: 1).close
  true
rescue Errno::ECONNREFUSED, Errno::EHOSTUNREACH, SocketError
  false
end

# Wait for port to become available
def wait_for_port(port, interval = 5)
  puts "Waiting for port #{port} to become available..."
  until port_open?(port)
    puts "Port #{port} is not available, retrying in #{interval} seconds..."
    sleep interval
  end
  puts "Port #{port} is now available."
end

# Helper to run a task with server and Cypress
def run_tier_task(cypress_cmd, wait_for_cypress: true)
  puts "--- Performing end-to-end @tier2 tests"
  server_pid = nil
  cypress_pid = nil

  # Early trap to handle SIGINT
  trap("INT") do
    puts "\nReceived SIGINT, terminating running processes..."
    if cypress_pid
      begin
        Process.kill("TERM", -cypress_pid)
      rescue
        nil
      end
      begin
        Process.wait(cypress_pid)
      rescue
        nil
      end
    end
    if server_pid
      begin
        Process.kill("TERM", -server_pid)
      rescue
        nil
      end
      begin
        Process.wait(server_pid)
      rescue
        nil
      end
    end
    exit 1
  end

  # Start the server
  server_pid = Process.spawn("trunk serve --no-autoreload --port=#{TRUNK_PORT}", pgroup: true)
  puts "Started trunk server with PID: #{server_pid}"

  # Wait for port
  wait_for_port(TRUNK_PORT.to_i)

  # Run Cypress
  cypress_pid = Process.spawn(cypress_cmd, pgroup: true)
  puts "Started Cypress with PID: #{cypress_pid}"

  if wait_for_cypress
    # Wait for Cypress to finish and capture exit status
    _, cypress_status = Process.wait2(cypress_pid)
    puts "Cypress finished with exit code: #{cypress_status.exitstatus}"

    # Kill the server after Cypress finishes
    puts "Killing trunk server..."
    begin
      Process.kill("TERM", -server_pid)
    rescue
      nil
    end
    begin
      Process.wait(server_pid)
    rescue
      nil
    end

    # Exit with Cypress’s status
    exit(cypress_status.exitstatus)
  else
    # For non-terminating Cypress (e.g., cypress open), just let it run
    # Server and Cypress will be killed via SIGINT when user is done
    puts "Cypress is running interactively. Press Ctrl+C to stop."
    # Wait indefinitely for SIGINT
    sleep
  end
end

def cmd_capture(command)
  output = ""

  IO.popen(command, err: [:child, :out]) do |io|
    io.each_line do |line|
      print line
      output << line
    end
  end

  # In Ruby, $? contains the status of the last executed child process
  unless $?.success?
    raise "Command '#{command}' failed with exit status: #{$?.exitstatus}\nOutput: #{output}"
  end

  output
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
  FileUtils.rm_rf %w[
    dist
    target
    cypress/screenshots
    node_modules
    .wrangler
    .build
  ]
end

desc "Format the source code"
task format: [:pnpm_install] do
  sh "pnpm exec prettier --write cypress/ src/scripts/"
  sh "standardrb --fix #{RUBY_SRC_FILES.join(" ")}"
  sh "cargo-fmt --all"
  sh "leptosfmt ./src"
end

desc "Run the Jest tests"
task jest_test: ".build/jest-test"

file ".build/jest-test" => JAVASCRIPT_SRC_FILES + ["jest.config.js"] do |t|
  puts "--- Performing jest unit tests"
  jest_output = cmd_capture("pnpm exec jest test")
  record_output(t, jest_output)
end

desc "Test the Rust code"
task rust_test: ".build/rust-test"

file ".build/rust-test" => CARGO_DEPS do |t|
  puts "--- Performing rust unit tests"
  nextest_output = cmd_capture("cargo-nextest nextest run")
  record_output(t, nextest_output)
end

desc "Run the unit tests"
task test_unit: [:jest_test, :rust_test]

desc "Audit the Rust code with cargo-audit"
task audit: ".build/audit"

file ".build/audit" => CARGO_DEPS do |t|
  audit_output = cmd_capture("cargo-audit audit")
  machete_output = cmd_capture("cargo machete")
  record_output(t, [audit_output, machete_output])
end

desc "Fix linting errors"
task :lint_fix do
  sh "standardrb --fix #{RUBY_SRC_FILES.join(" ")}"
  sh "cargo clippy --fix --allow-dirty --allow-staged"
end

desc "Build documentation in the build directory"
task build_docs: ".build/docs"

file ".build/docs" => GRAPHQL_SRC_FILES do |t|
  mkdir_p(".build")
  sh "cargo doc --document-private-items --target-dir #{t.name}"
end

desc "Install the JavaScript dependencies with 'pnpm'"
task pnpm_install: "node_modules"

file "node_modules" => ["pnpm-lock.yaml", "package.json"] do
  puts "--- Installing NPM dependencies"
  sh "pnpm install"
end

desc "Serve the built website locally"
task dev: [:deploy_mina_indexer] do
  ENV["GRAPHQL_URL"] = "http://localhost:8080/graphql"
  ENV["REST_URL"] = "http://localhost:8080"
  trap("INT") { Rake::Task["shutdown_mina_indexer"].invoke }
  sh "trunk serve --port=#{TRUNK_PORT} --open"
end

desc "Serve the built website locally against prod indexer"
task :dev_prod do
  ENV["GRAPHQL_URL"] = MINASEARCH_GRAPHQL
  ENV["REST_URL"] = MINASEARCH_REST
  sh "trunk serve --port=#{TRUNK_PORT} --open"
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

file ".build/check" => CARGO_DEPS do |t|
  check_output = cmd_capture("cargo check")
  record_output(t, check_output)
end

desc "Lint the Cypress test code (JavaScript)"
task lint_javascript: ".build/lint-javascript"

file ".build/lint-javascript" => CYPRESS_FILES + ["node_modules"] do |t|
  puts "--- Linting JS/TS"
  prettier_output = cmd_capture("pnpm exec prettier --check cypress/")
  record_output(t, prettier_output)
end

desc "Lint the Ruby code"
task lint_ruby: ".build/lint-ruby"

file ".build/lint-ruby" => RUBY_SRC_FILES do |t|
  puts "--- Linting ruby scripts"
  ruby_cw_output = cmd_capture("ruby -cw #{RUBY_SRC_FILES.join(" ")}")
  ruby_std_output = cmd_capture("standardrb --no-fix #{RUBY_SRC_FILES.join(" ")}")
  record_output(t, [ruby_cw_output, ruby_std_output])
end

desc "Lint the Rust code"
task lint_rust: ".build/lint-rust"

file ".build/lint-rust" => RUST_SRC_FILES + ["rustfmt.toml"] do |t|
  puts "--- Linting Rust code"
  cargo_fmt_out = cmd_capture("cargo-fmt --all --check")
  leptos_fmt_out = cmd_capture("leptosfmt --check ./src")
  clippy_out = cmd_capture("cargo clippy --all-targets --all-features -- -D warnings")
  record_output(t, [cargo_fmt_out, leptos_fmt_out, clippy_out])
end

desc "Build the dev version for front-end WASM bundle"
task dev_build: "target/debug"
file "target/debug" => CARGO_DEPS + ["Trunk.toml", "tailwind.config.js"] do
  puts "--- Building dev version"
  ENV["GRAPHQL_URL"] = "http://localhost:8080/graphql"
  ENV["REST_URL"] = "http://localhost:8080"
  cmd_capture("trunk build")
end

desc "Build the release version for front-end WASM bundle"
task release_build: "target/release"
file "target/release" => CARGO_DEPS + ["Trunk.toml", "tailwind.config.js"] do
  puts "--- Building release version"
  ENV["GRAPHQL_URL"] = MINASEARCH_GRAPHQL
  ENV["REST_URL"] = MINASEARCH_REST
  cmd_capture("trunk build --release --filehash true")
end

desc "Lint all source code"
task lint: [:pnpm_install, :audit, :lint_javascript, :lint_ruby, :lint_rust]

desc "Run the Tier1 tests"
task tier1: [:lint, :test_unit, :dev_build]

desc "Invoke the Tier2 regression suite (non-interactive)"
task tier2: [:tier1, :pnpm_install, :deploy_mina_indexer] do
  run_tier_task("pnpm exec cypress run -r list -q", wait_for_cypress: true)
end

desc "Invoke the Tier2 regression suite (interactive)"
task :t2_i do
  run_tier_task("pnpm exec cypress open", wait_for_cypress: false)
end

desc "Print all tasks and their dependencies as a tree"
task :print_tree do
  def print_task_tree(task, prefix = "", is_last = true, visited = Set.new)
    task_name = task.name
    return if visited.include?(task_name)

    visited.add(task_name)
    puts "#{prefix}#{is_last ? "└── " : "├── "}#{task_name}"

    prereqs = task.prerequisites
    prereqs.each_with_index do |prereq, index|
      next unless Rake::Task.task_defined?(prereq)
      is_last_prereq = (index == prereqs.size - 1)
      new_prefix = prefix + (is_last ? "    " : "│   ")
      print_task_tree(Rake::Task[prereq], new_prefix, is_last_prereq, visited)
    end
  end

  all_tasks = Rake.application.tasks
  all_prereqs = all_tasks.flat_map(&:prerequisites).uniq
  root_tasks = all_tasks.reject { |t| all_prereqs.include?(t.name) }

  puts "Task Dependency Tree (All Root Tasks):"
  if root_tasks.empty?
    puts "No root tasks found."
  else
    root_tasks.each_with_index do |task, index|
      is_last_root = (index == root_tasks.size - 1)
      puts "\nRoot Task ##{index + 1}:" unless root_tasks.size == 1
      print_task_tree(task, "", is_last_root)
    end
  end

  orphaned_tasks = all_tasks - root_tasks - all_tasks.flat_map { |t|
                                              t.prerequisites.map { |p|
                                                begin
                                                  Rake::Task[p]
                                                rescue
                                                  nil
                                                end
                                              }
                                            }.compact
  unless orphaned_tasks.empty?
    puts "\nOrphaned Tasks (no dependencies or dependents):"
    orphaned_tasks.each { |task| puts "  - #{task.name} (#{task.full_comment || "No description"})" }
  end
end
