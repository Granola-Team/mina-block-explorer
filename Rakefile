require "open3"
require "fileutils"
require "socket"

# Constants and environment variables
TOP = __dir__
TRUNK_PORT = rand(5170..5179).to_s
IDXR_PORT = 8090.to_s
ENV["BROWSER_PATH"] = `which google-chrome-stable`.chomp
ENV["APP_PORT"] = TRUNK_PORT
ENV["RUSTFLAGS"] = "--cfg=web_sys_unstable_apis"
ENV["VERSION"] = `git rev-parse --short=8 HEAD`.chomp
ENV["INDEXER_VERSION"] = `cd lib/mina-indexer && git rev-parse --short=8 HEAD`.chomp
ENV["CARGO_HOME"] = "#{TOP}/rust/.cargo"
ENV["VOLUMES_DIR"] ||= "/mnt" if Dir.exist?("/mnt")
ENV["GRAPHQL_URL"] = "http://localhost:#{IDXR_PORT}/graphql"
ENV["REST_URL"] = "http://localhost:#{IDXR_PORT}"
GRAPHQL_SRC_FILES = Dir.glob("graphql/**/*.graphql")
RUST_SRC_FILES = Dir.glob("rust/**/*.rs").reject { |file| file.start_with?("rust/.cargo", "rust/target") }
CARGO_DEPS = RUST_SRC_FILES + GRAPHQL_SRC_FILES + ["rust/Cargo.toml", "rust/Cargo.lock", "rust/.cargo/audit.toml", "rust/.cargo/config.toml"]
RUBY_SRC_FILES = Dir.glob("**/*.rb").reject { |file| file.start_with?("lib/") } + ["Rakefile"]
JAVASCRIPT_SRC_FILES = Dir.glob("trunk/scripts_tests/**")
MINASEARCH_GRAPHQL = "https://api.minasearch.com/graphql"
MINASEARCH_REST = "https://api.minasearch.com"
DEV_BUILD_TARGET = ".build/dev_build"
RELEASE_BUILD_TARGET = ".build/release_build"
IDXR_FOLDER = "#{TOP}/lib/mina-indexer"

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
  FileUtils.mkdir_p("#{TOP}/.build")
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

# Helper to run a task with server and RSpec
def run_tier_task(rspec_cmd)
  puts "--- Performing end-to-end tier2 tests"
  server_pid = nil
  rspec_pid = nil

  # Early trap to handle SIGINT
  trap("INT") do
    puts "\nReceived SIGINT, terminating running processes..."
    if rspec_pid
      begin
        Process.kill("INT", -rspec_pid)
      rescue
        nil
      end
      begin
        Process.wait(rspec_pid)
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
  server_pid = Dir.chdir("#{TOP}/trunk") do
    Process.spawn("trunk serve --no-autoreload --port=#{TRUNK_PORT} --dist=../#{DEV_BUILD_TARGET}", pgroup: true)
  end
  puts "Started trunk server with PID: #{server_pid}"

  # Wait for port
  wait_for_port(TRUNK_PORT.to_i)

  # Run RSpec
  rspec_pid = Process.spawn(rspec_cmd, pgroup: true)
  puts "Started RSpec (bundler) with PID: #{rspec_pid}"

  # Wait for RSpec to finish and capture exit status
  _, rspec_status = Process.wait2(rspec_pid)
  puts "RSpec (bundler) finished with exit code: #{rspec_status.exitstatus}"

  # Kill the server
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

  # Important! Exit with RSpec’s status.
  exit(rspec_status.exitstatus)
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

task clone_mina_indexer: IDXR_FOLDER.to_s
file IDXR_FOLDER.to_s do
  puts "--- Cloning indexer"
  sh "git submodule update --init"
end

desc "Deploy mina-indexer"
task deploy_mina_indexer: [:clone_mina_indexer] do
  ensure_env_vars(%w[VOLUMES_DIR], "Cannot deploy mina indexer")
  puts "--- Deploying mina-indexer at #{ENV["INDEXER_VERSION"]}"
  Dir.chdir("#{TOP}/lib/mina-indexer") do
    sh "nix develop --command rake build:dev"
    sh "nix develop --command rake \"bin:stage_and_start_v2[#{Dir.pwd}/rust/target/debug/mina-indexer,361000,#{IDXR_PORT}]\""
  end
end

task :clean_test do
  FileUtils.rm_rf "ruby/spec-screenshots"
end

task :clean_node_modules do
  FileUtils.rm_rf "pnpm/node_modules"
end

task :clean_build do
  FileUtils.rm_rf ".build"
end

task :clean_target do
  FileUtils.rm_rf "rust/target"
end

desc "Clean the repo of built artifacts"
task clean: [:clean_test, :clean_node_modules, :clean_build, :clean_target]

desc "Format the source code"
task format: ["pnpm/node_modules"] do
  Dir.chdir("#{TOP}/pnpm") do
    sh "pnpm exec prettier --write ../trunk/scripts"
  end
  sh "standardrb --fix #{RUBY_SRC_FILES.join(" ")}"
  sh "rustfmt --edition 2024 #{RUST_SRC_FILES.join(" ")}"
  Dir.chdir("#{TOP}/rust") do
    sh "leptosfmt ."
    sh "cargo clippy --fix --allow-dirty --allow-staged"
  end
end

desc "Run the Jest tests"
task jest_test: ".build/jest-test"

file ".build/jest-test" => JAVASCRIPT_SRC_FILES + ["pnpm/jest.config.js"] do |t|
  puts "--- Performing jest unit tests"
  jest_output = Dir.chdir("#{TOP}/pnpm") do
    cmd_capture("pnpm exec jest test")
  end
  record_output(t, jest_output)
end

desc "Test the Rust code"
task rust_test: ".build/rust-test"

file ".build/rust-test" => CARGO_DEPS do |t|
  puts "--- Performing rust unit tests"
  nextest_output = Dir.chdir("#{TOP}/rust") do
    cmd_capture("cargo-nextest nextest run")
  end
  record_output(t, nextest_output)
end

desc "Run the unit tests"
task test_unit: [:jest_test, :rust_test]

desc "Audit the Rust code with cargo-audit"
task audit: ".build/audit"

file ".build/audit" => CARGO_DEPS do |t|
  audit_output = Dir.chdir("#{TOP}/rust") do
    cmd_capture("cargo-audit audit")
  end
  machete_output = Dir.chdir("#{TOP}/rust") do
    cmd_capture("cargo machete")
  end
  record_output(t, [audit_output, machete_output])
end

desc "Build documentation in the build directory"
task build_docs: ".build/docs"

file ".build/docs" => GRAPHQL_SRC_FILES do |t|
  mkdir_p(".build")
  Dir.chdir("#{TOP}/rust") do
    sh "cargo doc --document-private-items --target-dir ../#{t.name}"
  end
end

task bundle_install: ".build/bundle"
file ".build/bundle" => ["ruby/Gemfile", "ruby/Gemfile.lock"] do |t|
  puts "--- Installing Ruby dependencies"
  Dir.chdir("#{TOP}/ruby") do
    sh "bundle install"
  end
end

file "pnpm/node_modules" => ["pnpm/pnpm-lock.yaml", "pnpm/package.json"] do
  puts "--- Installing NPM dependencies"
  Dir.chdir("#{TOP}/pnpm") do
    sh "pnpm install"
  end
end

desc "Serve the built website locally"
task dev: [:deploy_mina_indexer, :dev_build] do
  trap("INT") { Rake::Task["shutdown_mina_indexer"].invoke }
  Dir.chdir("#{TOP}/trunk") do
    sh "trunk serve --port=#{TRUNK_PORT} --open --dist=../#{DEV_BUILD_TARGET}"
  end
end

desc "Serve the built website locally against prod indexer"
task dev_prod: [:release_build] do
  Dir.chdir("#{TOP}/trunk") do
    sh "trunk serve --port=#{TRUNK_PORT} --open --dist=../#{RELEASE_BUILD_TARGET}"
  end
end

task :check_tokens do
  puts "--- Checking presence of tokens"
  ensure_env_vars(%w[CLOUDFLARE_ACCOUNT_ID CLOUDFLARE_API_TOKEN], "Cannot publish")
end

desc "Publish the website to production"
task publish: [:check_tokens, "pnpm/node_modules", :release_build] do
  puts "--- Publishing"
  puts "Publishing version #{ENV["VERSION"]}"
  Dir.chdir("#{TOP}/pnpm") do
    sh "pnpx wrangler pages deploy --branch main"
  end
end

desc "Use 'cargo check' to verify buildability"
task check: ".build/check"

file ".build/check" => CARGO_DEPS do |t|
  check_output = Dir.chdir("#{TOP}/rust") do
    cmd_capture("cargo check")
  end
  record_output(t, check_output)
end

desc "Lint the Ruby code"
task lint_ruby: ".build/lint-ruby"

file ".build/lint-ruby" => RUBY_SRC_FILES do |t|
  puts "--- Linting ruby scripts"
  ruby_cw_output = cmd_capture("ruby -cw #{RUBY_SRC_FILES.join(" ")}")
  ruby_std_output = cmd_capture("standardrb --no-fix #{RUBY_SRC_FILES.join(" ")}")
  record_output(t, [ruby_cw_output, ruby_std_output])
end

desc "Lint the JavaScript code"
task lint_js: ".build/lint-js"

file ".build/lint-js" => JAVASCRIPT_SRC_FILES + ["pnpm/node_modules"] do |t|
  puts "--- Linting the JavaScript code"
  prettier_output = Dir.chdir("#{TOP}/pnpm") do
    cmd_capture("pnpm exec prettier --check ../trunk/scripts")
  end
  record_output(t, prettier_output)
end

desc "Lint the Rust code"
task lint_rust: ".build/lint-rust"

file ".build/lint-rust" => RUST_SRC_FILES do |t|
  puts "--- Linting Rust code"
  leptos_fmt_out = cmd_capture("leptosfmt --check #{RUST_SRC_FILES.join(" ")}")
  fmt_out = cmd_capture("rustfmt --edition 2024 --check #{RUST_SRC_FILES.join(" ")}")
  clippy_out = Dir.chdir("#{TOP}/rust") do
    cmd_capture("cargo clippy --no-deps -- -D warnings")
  end
  record_output(t, [fmt_out, leptos_fmt_out, clippy_out])
end

desc "Build the dev version for front-end WASM bundle"
task dev_build: DEV_BUILD_TARGET
file DEV_BUILD_TARGET => CARGO_DEPS + ["trunk/Trunk.toml", "trunk/tailwind.config.js"] do |t|
  puts "--- Building dev version"
  Dir.chdir("#{TOP}/trunk") do
    sh "trunk build --dist=../#{t.name}"
  end
end

desc "Build the release version for front-end WASM bundle"
task release_build: RELEASE_BUILD_TARGET
file RELEASE_BUILD_TARGET => CARGO_DEPS + ["trunk/Trunk.toml", "trunk/tailwind.config.js"] do |t|
  puts "--- Building release version"
  ENV["GRAPHQL_URL"] = MINASEARCH_GRAPHQL
  ENV["REST_URL"] = MINASEARCH_REST
  Dir.chdir("#{TOP}/trunk") do
    sh "trunk build --release --filehash true --dist=../#{t.name}"
  end
end

desc "Lint all source code"
task lint: [:audit, :lint_ruby, :lint_rust, :lint_js]

desc "Run the Tier1 tests"
task tier1: [:dev_build, :lint, :test_unit]

# Defining common prerequisites for Tier2 tasks
tier2_prerequisites = [:clean_test, :tier1, "pnpm/node_modules", :bundle_install, :shutdown_mina_indexer, :deploy_mina_indexer]

# Helper method to run tier tasks with common setup and teardown
def run_tier2_task(rspec_command)
  trap("INT") { Rake::Task["shutdown_mina_indexer"].invoke }
  run_tier_task(rspec_command)
  Rake::Task["shutdown_mina_indexer"].invoke
end

desc "Invoke the Tier2 regression suite (non-interactive)"
task tier2: tier2_prerequisites do
  Dir.chdir("#{TOP}/ruby") do
    run_tier2_task("bundle exec rspec")
  end
end

desc "Retry the Tier2 regression suite (non-interactive)"
task tier2_retry: tier2_prerequisites do
  Dir.chdir("#{TOP}/ruby") do
    run_tier2_task("bundle exec rspec --only-failures")
  end
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
