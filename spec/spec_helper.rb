require "capybara/rspec"
require "capybara/cuprite"
require "rspec/retry"
require_relative "support/constants"
require_relative "support/capybara_helpers"

RSpec.configure do |config|
  config.include CapybaraHelpers, type: :system
end

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
  config.example_status_persistence_file_path = ".build/spec_results.txt"
  config.disable_monkey_patching!
  config.warnings = true
  config.default_formatter = "doc" if config.files_to_run.one?
  config.profile_examples = 10
  Kernel.srand config.seed

  # show retry status in spec process
  config.verbose_retry = true
  # show exception that triggers a retry if verbose_retry is set to true
  config.display_try_failure_messages = true

  # run retry only on features
  config.around :each do |ex|
    ex.run_with_retry retry: 3
  end

  config.after(:each) do |example|
    if example.exception && example.metadata[:type] == :system
      timestamp = Time.now.strftime("%Y%m%d_%H%M%S")
      process_id = ENV["TEST_ENV_NUMBER"] || "0"
      filename = "tmp/screenshot-failed-#{timestamp}-#{process_id}.png"
      page.save_screenshot(filename) # standard:disable all
    end
  end
end
