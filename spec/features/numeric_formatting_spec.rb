require "spec_helper"

RSpec.describe "Number or currency", type: :system do
  items = [
    {
      url: "/tokens",
      heading: "Tokens",
      tests: [
        {name: "supply column", selector: '[data-test="tokens-table"] tr:has(:not(th)) td:nth-child(2)', type: "number"}
      ]
    },
    {
      url: "/blocks",
      heading: "Blocks",
      tests: [
        {name: "overview", selector: "#blockchainLength", type: "number"},
        {name: "height column", selector: '[data-test="blocks-table"] tr:has(:not(th)) td:first-child', type: "number"},
        {name: "slot column", selector: '[data-test="blocks-table"] tr:has(:not(th)) td:nth-child(3)', type: "number"}
      ]
    },
    {
      url: "/blocks/#{GENESIS_BLOCK_BLOCK_HASH}/spotlight",
      heading: "Block Spotlight",
      tests: [
        {name: "coinbase", selector: "table tr:nth-child(5) td:first-child", type: "currency"},
        {name: "SNARK Fees", selector: "table tr:nth-child(7) td:first-child", type: "currency"},
        {name: "Transaction Fees", selector: "table tr:nth-child(11) td:first-child", type: "currency"}
      ]
    },
    {
      url: "/blocks/#{BLOCK_WITH_ALL_ACTIVITY}/commands/user",
      heading: "User Commands",
      tests: [
        {name: "fee", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:nth-child(7)', type: "currency"},
        {name: "amount", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:last-child', type: "currency"}
      ]
    },
    {
      url: "/blocks/#{BLOCK_WITH_ALL_ACTIVITY}/commands/internal",
      heading: "Internal Commands",
      tests: [
        {name: "fee", selector: '[data-test="internal-commands-table"] tr:has(:not(th)) td:nth-child(2)', type: "currency"}
      ]
    },
    {
      url: "/blocks/#{BLOCK_WITH_ALL_ACTIVITY}/snark-jobs",
      heading: "SNARK Jobs",
      tests: [
        {name: "fee", selector: '[data-test="snark-jobs-table"] tr:has(:not(th)) td:last-child', type: "currency"}
      ]
    },
    {
      url: "/commands/user",
      heading: "User Commands",
      tests: [
        {name: "height column", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:first-child', type: "number"},
        {name: "nonce column", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:nth-child(8)', type: "number"},
        {name: "fee column", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:nth-child(9)', type: "currency"},
        {name: "amount column", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:nth-child(10)', type: "currency"}
      ]
    },
    {
      url: "/commands/internal",
      heading: "Internal Commands",
      tests: [
        {name: "height column", selector: '[data-test="internal-commands-table"] tr:has(:not(th)) td:first-child', type: "number"},
        {name: "fee column", selector: '[data-test="internal-commands-table"] tr:has(:not(th)) td:nth-child(4)', type: "currency"}
      ]
    },
    {
      url: "/addresses/accounts",
      heading: "MINA Accounts",
      tests: [
        {name: "balance column", selector: '[data-test="mina-accounts-table"] tr:has(:not(th)) td:nth-child(4)', type: "currency"},
        {name: "nonce column", selector: '[data-test="mina-accounts-table"] tr:has(:not(th)) td:nth-child(5)', type: "number"}
      ]
    },
    {
      url: "/addresses/accounts/#{COMMAND_SNARK_BLOCK_ACTIVITY_ADDRESS}",
      heading: "User Commands",
      tests: [
        {name: "user command height column", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:first-child', type: "number"},
        {name: "user command nonce column", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:nth-child(3)', type: "number"},
        {name: "user command amount column", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:last-child span span:first-child', type: "currency"},
        {name: "user command fee column", selector: '[data-test="user-commands-table"] tr:has(:not(th)) td:last-child span span:last-child', type: "currency"}
      ]
    },
    {
      url: "/addresses/accounts/#{COMMAND_SNARK_BLOCK_ACTIVITY_ADDRESS}/snark-jobs",
      heading: "SNARK Jobs",
      tests: [
        {name: "snark work height column", selector: '[data-test="snark-jobs-table"] tr:has(:not(th)) td:first-child', type: "number"},
        {name: "snark work fee column", selector: '[data-test="snark-jobs-table"] tr:has(:not(th)) td:last-child', type: "currency"}
      ]
    },
    {
      url: "/addresses/accounts/#{COMMAND_SNARK_BLOCK_ACTIVITY_ADDRESS}/block-production",
      heading: "Block Production",
      tests: [
        {name: "block production height column", selector: '[data-test="block-production-table"] tr:has(:not(th)) td:first-child', type: "number"},
        {name: "block production slot column", selector: '[data-test="block-production-table"] tr:has(:not(th)) td:nth-child(3)', type: "number"},
        {name: "block production coinbase column", selector: '[data-test="block-production-table"] tr:has(:not(th)) td:nth-child(6)', type: "currency"}
      ]
    },
    {
      url: "/snarks",
      heading: "SNARKs",
      tests: [
        {name: "height column", selector: '[data-test="snarks-table"] tr:has(:not(th)) td:first-child', type: "number"},
        {name: "fee column", selector: '[data-test="snarks-table"] tr:has(:not(th)) td:last-child', type: "currency"}
      ]
    },
    {
      url: "/staking-ledgers?epoch=0&q-key=#{ROMEK_ADDRESS}",
      heading: "Staking Ledger - Epoch 0",
      tests: [
        {name: "stake column", selector: '[data-test="staking-ledger-epoch-0-table"] tr:has(:not(th)) td:nth-child(3)', type: "currency"}
      ]
    }
  ]

  items.each do |item|
    item[:tests].each do |test|
      it "on page #{item[:url]} is formatted correctly for '#{test[:name]}'" do
        visit item[:url]
        wait_until_table_loaded(item[:heading])
        # page.save_screenshot("tmp/#{test[:name]}.png")
        all(test[:selector]).each do |element|
          is_numeric?(element)
        end
      end
    end
  end
end
