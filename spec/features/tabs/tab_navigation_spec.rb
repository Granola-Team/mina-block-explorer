require "spec_helper"

TABS = [
  {
    page: "/commands/internal",
    tab: "User Commands",
    expected_url: "/commands/user"
  },
  {
    page: "/commands/user",
    tab: "Internal Commands",
    expected_url: "/commands/internal"
  },
  {
    page: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/snark-jobs",
    tab: "User Commands",
    expected_url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/commands/user"
  },
  {
    page: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}",
    tab: "SNARK Jobs",
    expected_url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/snark-jobs"
  },
  {
    page: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}",
    tab: "Block Production",
    expected_url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/block-production"
  },
  {
    page: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}",
    tab: "Internal Commands",
    expected_url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/commands/internal"
  }
]

RSpec.describe "Tab navigation", type: :system do
  TABS.each do |test_case|
    it "links to page #{test_case[:expected_url]} from '#{test_case[:tab]}' tab" do
      visit test_case[:page]

      # Find the active and target tabs
      active_tab = find("a.active", match: :first)
      target_tab = find("a#{tab_selector(test_case[:tab])}.inactive", text: test_case[:tab].upcase, match: :first)

      # Verify the active tab does not contain the target tab's text
      expect(active_tab.text).not_to include(test_case[:tab].upcase), "Expected active tab to not contain '#{test_case[:tab].upcase}', but it did"

      # Click the target tab
      target_tab.click

      # Verify the URL
      expect(page.current_url).to include(test_case[:expected_url]), "Expected URL to include '#{test_case[:expected_url]}', but was '#{page.current_url}'"

      # Verify the active tab now contains the target tab's text
      new_active_tab = find("a.active", match: :first)
      expect(new_active_tab.text).to include(test_case[:tab].upcase), "Expected new active tab to contain '#{test_case[:tab].upcase}', but it did not"
    end
  end
end
