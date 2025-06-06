require "spec_helper"

RSpec.describe "Internal commands page navigation", type: :system do
  pages = [
    {
      origin: "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}/#{Constants::GENESIS_ACCOUNT_PK}/commands/internal",
      dest: "internal commands",
      href: "/commands/internal?q-recipient=#{Constants::GENESIS_ACCOUNT_PK}"
    }
  ]

  pages.each do |test_case|
    it "is navigated to from #{test_case[:origin]}" do
      visit test_case[:origin]

      wait_until_table_loaded("Internal Commands")

      # Click the link with text "See all internal commands"
      find("a", text: "SEE ALL INTERNAL COMMANDS").click

      # Verify the URL contains the expected href
      expect(page.current_url).to include(test_case[:href]), "Expected URL to include '#{test_case[:href]}', but was #{page.current_url}"

      # Verify the "Recipient" column in the "Internal Commands" table contains GENESIS_ACCOUNT_PK
      wait_until_table_loaded("Internal Commands")
      recipient_cells = all(table_column_selector("Internal Commands", "Recipient".upcase))
      recipient_cells.each do |cell|
        # Join split text if necessary (as per your previous issue)
        cleaned_text = cell.text.gsub(/[\n+-]/, "")
        expect(cleaned_text).to include(Constants::GENESIS_ACCOUNT_PK), "Expected 'Recipient' column to contain '#{Constants::GENESIS_ACCOUNT_PK}', but found '#{cleaned_text}'"
      end
    end
  end

  it "is navigated to from nav menu" do
    visit "/"
    wait_until_table_loaded("Blocks")
    click_nav_menu_item(["Transactions", "Internal Commands"])
    expect(page.current_path).to match(/\/commands\/internal/), "Expected URL to include '/commands/internal', but was #{page.current_path}"
  end
end
