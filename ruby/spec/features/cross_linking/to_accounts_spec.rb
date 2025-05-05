require "spec_helper"

RSpec.describe "Account page navigation", type: :system do
  pages = [
    {
      origin: "/analytics/staker-leaderboard",
      column: "Delegate",
      tableHeader: "Staker Leaderboard"
    },
    {
      origin: "/addresses/accounts",
      column: "Public Key",
      tableHeader: "MINA Token Accounts"
    },
    {
      origin: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}",
      column: "Counterparty",
      tableHeader: "User Commands"
    },
    {
      origin: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/block-production",
      column: "Coinbase Receiver",
      tableHeader: "Block Production"
    },
    {
      origin: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/delegations",
      column: "Public Key",
      tableHeader: "Delegations"
    },
    {
      origin: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/delegations",
      column: "Username",
      tableHeader: "Delegations"
    },
    {origin: "/snarks", column: "Prover", tableHeader: "SNARKs"},
    {
      origin: "/staking-ledgers?epoch=1",
      column: "Account",
      tableHeader: "Staking Ledger - Epoch 1"
    },
    {
      origin: "/staking-ledgers?epoch=1&q-delegate=#{Constants::STAKER_ADDRESS}&sort-dir=STAKE_ASC",
      column: "Delegate",
      tableHeader: "Staking Ledger - Epoch 1"
    },
    {
      origin: "/commands/user",
      column: "From",
      tableHeader: "User Commands"
    },
    {
      origin: "/commands/user",
      column: "To",
      tableHeader: "User Commands"
    },
    {
      origin: "/blocks/#{Constants::FIRST_BLOCK_WITH_SNARK_WORK}/commands/user",
      column: "From",
      tableHeader: "User Commands"
    },
    {
      origin: "/blocks/#{Constants::FIRST_BLOCK_WITH_SNARK_WORK}/commands/user",
      column: "To",
      tableHeader: "User Commands"
    },
    {
      origin: "/blocks/#{Constants::FIRST_BLOCK_WITH_SNARK_WORK}/snark-jobs",
      column: "Prover",
      tableHeader: "SNARK Jobs"
    },
    {
      origin: "/blocks/#{Constants::FIRST_BLOCK_WITH_SNARK_WORK}/commands/internal",
      column: "Recipient",
      tableHeader: "Internal Commands"
    }
  ]

  pages.each do |item|
    it "is navigated to from #{item[:origin]} by clicking link in '#{item[:column]}'" do
      visit item[:origin]
      wait_until_table_loaded(item[:tableHeader])
      click_link_in_table_column(item[:tableHeader], item[:column].upcase, 1)
      expect(page.current_path).to match(/\/accounts\//), "Expected URL to include '/accounts/', but was #{page.current_path}"
    end
  end

  it "is navigated to from /tokens by clicking link in 'Holders'" do
    visit "/tokens"

    wait_until_table_loaded("Tokens")

    # Click the "Holders" link in the 2nd row
    click_link_in_table_column("Tokens", "Holders".upcase, 2)

    # Verify the URL includes the expected parameters
    expect(page.current_url).to include("/addresses/accounts?q-token=#{Constants::MINU_TOKEN_ADDRESS}"), "Expected URL to include '/addresses/accounts?q-token=#{Constants::MINU_TOKEN_ADDRESS}', but was '#{page.current_url}'"

    # Verify the "MINU Token Accounts" table has 1 row
    wait_until_table_loaded("MINU Token Accounts")
    table_rows = get_table_rows("MINU Token Accounts")
    expect(table_rows.count).to eq(1), "Expected 'MINU Token Accounts' table to have 1 row, but found #{table_rows.count}"

    # Verify table metadata (1 of 1)
    metadata = get_table_metadata("MINU Token Accounts")
    expect(metadata[0]).to eq(1), "Expected metadata 'x' in 'x of y' to be 1, but was #{metadata[0]}"
    expect(metadata[1]).to eq(1), "Expected metadata 'y' in 'x of y' to be 1, but was #{metadata[1]}"
  end
end
