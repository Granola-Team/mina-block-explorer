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

  it "is navigated to with correct url params from /tokens page by clicking link in 'Transactions'" do
    visit "/tokens"

    # Click the "Transactions" link in the 2nd row (Cypress index 1 = Capybara row 2)
    click_link_in_table_column("Tokens", "Transactions".upcase, 2)

    # Verify the URL includes the expected parameters
    expect(page.current_url).to include("/commands/user"), "Expected URL to include '/commands/user', but was '#{page.current_url}'"
    expect(page.current_url).to include("q-token=#{Constants::MINU_TOKEN_ADDRESS}"), "Expected URL to include 'q-token=#{Constants::MINU_TOKEN_ADDRESS}', but was '#{page.current_url}'"
    expect(page.current_url).to include("q-status=All"), "Expected URL to include 'q-status=All', but was '#{page.current_url}'"

    # Verify the "User Commands (MINU)" table has 1 row
    wait_until_table_loaded("User Commands")
    table_rows = get_table_rows("User Commands")
    expect(table_rows.count).to eq(1), "Expected 'User Commands (#{Constants::MINU_SYMBOL})' table to have 1 row, but found #{table_rows.count}"

    # Verify table metadata (1 of 1)
    metadata = get_table_metadata("User Commands (#{Constants::MINU_SYMBOL})")
    expect(metadata[0]).to eq(1), "Expected metadata 'x' in 'x of y' to be 1, but was #{metadata[0]}"
    expect(metadata[1]).to eq(1), "Expected metadata 'y' in 'x of y' to be 1, but was #{metadata[1]}"
  end
end
