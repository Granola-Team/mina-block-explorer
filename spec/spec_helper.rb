require "capybara/rspec"
require "capybara/cuprite"
require_relative "support/test_constants"

raise "APP_PORT environment variable is not set" unless ENV.key?("APP_PORT")

# Configure Cuprite driver
Capybara.register_driver :cuprite do |app|
  Capybara::Cuprite::Driver.new(
    app,
    headless: true, # Headless for CI/build machine
    browser_options: {"no-sandbox": true}, # Required for CI
    window_size: [1920, 1080] # Consistent viewport
  )
end

Capybara.default_driver = :cuprite
Capybara.javascript_driver = :cuprite
Capybara.default_max_wait_time = 5 # Adjust for async apps

Capybara.app_host = "http://localhost:#{ENV["APP_PORT"]}" # Base URL
# Capybara.app_host = "http://localhost:5171"

# RSpec configuration
RSpec.configure do |config|
  config.expect_with :rspec do |expectations|
    expectations.include_chain_clauses_in_custom_matcher_descriptions = true
  end
  config.mock_with :rspec do |mocks|
    mocks.verify_partial_doubles = true
  end
  config.shared_context_metadata_behavior = :apply_to_host_groups
  config.filter_run_when_matching :focus
  config.example_status_persistence_file_path = ".build/rspec_status.txt"
  config.disable_monkey_patching!
  config.warnings = true
  config.default_formatter = "doc" if config.files_to_run.one?
  config.profile_examples = 10
  config.order = :random
  Kernel.srand config.seed
end
