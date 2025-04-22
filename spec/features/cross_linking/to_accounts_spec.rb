require "spec_helper"

RSpec.describe "Account page navigation", type: :system do
  pages = [
    {
      origin: "/analytics/staker-leaderboard",
      column: "Public Key",
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
      column: "Key",
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
      wait_until_table_loaded(item[:tableHeader], wait: 10)
      click_link_in_table_column(item[:tableHeader], item[:column].upcase, 1)
      expect(page.current_path).to match(/\/accounts\//), "Expected URL to include '/accounts/', but was #{page.current_path}"
    end
  end
end
