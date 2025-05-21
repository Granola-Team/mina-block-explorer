require "spec_helper"

RSpec.describe "Staking ledger page navigation", type: :system do
  pages = [
    {
      origin: "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}/#{Constants::GENESIS_ACCOUNT_PK}/delegations",
      dest: "staking-ledgers",
      href: "q-delegate=#{Constants::GENESIS_ACCOUNT_PK}"
    }
  ]

  pages.each do |test_case|
    it "is navigated to from #{test_case[:dest]}" do
      visit test_case[:origin]

      wait_until_table_loaded("Delegations")

      # Click the link with text "See all delegators"
      find("a", text: "See all delegators".upcase).click

      # Verify the URL contains the expected href
      expect(page.current_url).to include(test_case[:href]), "Expected URL to include '#{test_case[:href]}', but was #{page.current_url}"

      # Verify the "Delegate" column in the "Staking Ledger - Epoch 1" table contains GENESIS_ACCOUNT_PK or "Self"
      wait_until_table_loaded("Staking Ledger - Epoch 0")
      delegate_cells = all(table_column_selector("Staking Ledger - Epoch 0", "Delegate".upcase))
      delegate_cells.each do |cell|
        cleaned_text = cell.text.gsub(/[\n+-]/, "")
        expect(cleaned_text).to include(Constants::GENESIS_ACCOUNT_PK), "Expected 'Delegate' column to contain '#{Constants::GENESIS_ACCOUNT_PK}', but found '#{cleaned_text}'"
      end
    end
  end

  it "is navigated to from nav menu" do
    visit "/"
    wait_until_table_loaded("Blocks")
    click_nav_menu_item(["Staking"])
    expect(page.current_path).to match(/\/staking-ledgers/), "Expected URL to include '/staking-ledgers', but was #{page.current_path}"
  end
end
