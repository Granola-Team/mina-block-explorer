require "spec_helper"

RSpec.describe "Transaction spotlight navigation", type: :system do
  pages = [
    {origin: "/commands", column: "Txn Hash", tableHeader: "User Commands"},
    {
      origin: "/blocks/#{Constants::APPLIED_TXN_BLOCK_STATE_HASH}/commands/user",
      column: "Hash",
      tableHeader: "User Commands"
    },
    {
      origin: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}",
      column: "Txn Hash",
      tableHeader: "User Commands"
    },
    {
      origin: "/tokens",
      column: "Transactions",
      tableHeader: "Tokens"
    }
  ]

  pages.each do |test_case|
    it "is navigated to from #{test_case[:origin]} by clicking link in '#{test_case[:column]}'" do
      visit test_case[:origin]
      wait_until_table_loaded(test_case[:tableHeader])
      click_link_in_table_column(test_case[:tableHeader], test_case[:column].upcase, 1)
      expect(page.current_url).to include("/commands/"), "Expected URL to include '/commands/', but was #{page.current_url}"
    end
  end

  it "is navigated to with correct url params from /commands/user page by clicking link in 'Txn Hash'" do
    visit "/commands/user?q-txn-hash=#{Constants::FIRST_TXN_HASH}"
    wait_until_table_loaded("User Commands")

    click_link_in_table_column("User Commands", "Txn Hash".upcase, 1)

    wait_until_spotlight_loaded

    within("section#spotlight-section table") do
      canonical = find("tr", text: "Canonical")
      data = canonical.find("td", match: :first)
      expect(data.text).to eq("true"), "Expected 'Canonical' to be 'true', but found '#{data.text}'"
    end
  end
end
