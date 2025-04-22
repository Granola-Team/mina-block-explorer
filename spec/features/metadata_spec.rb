require "spec_helper"

RSpec.describe "Accounts metadata", type: :system do
  [
    {
      url: "/addresses/accounts?q-token=#{NFT_TOKEN_ID}",
      table_header: "NFT Token Accounts",
      metadata: [1, TOTAL_NUM_NFT_HOLDERS, TOTAL_NUM_ACCOUNTS]
    }
  ].each do |item|
    it "is correct for #{item[:url]}" do
      visit item[:url]
      wait_until_table_loaded(item[:table_header])

      # Assert row count (first metadata value)
      table_selector = "table[data-test='#{to_kebab_case(item[:table_header])}-table']"
      row_count = page.all("#{table_selector} tr:not(:has(th))", wait: 1).count
      expect(row_count).to eq(item[:metadata][0]), "Expected #{item[:metadata][0]} rows for '#{item[:table_header]}', but found #{row_count}"

      # Parse metadata using get_by_sel
      metadata_selector = "metadata-#{to_kebab_case(item[:table_header])}"
      metadata_element = get_by_sel(metadata_selector)
      metadata_text = metadata_element.text
      # Parse "x of y of z" format, removing commas for number conversion
      metadata_numbers = metadata_text.scan(/\d[\d,]*/).map { |num| num.delete(",").to_i }

      # Assert metadata values
      expect(metadata_numbers[1]).to eq(item[:metadata][1]), "Expected total of #{item[:metadata][1]} for '#{item[:table_header]}', but found #{metadata_numbers[1]}"
    end
  end
end

RSpec.describe "User command metadata", type: :system do
  row_limit = 25
  [
    {
      url: "/commands/user",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUMBER_APPLIED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=All&txn-type=Canonical",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUM_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=All&txn-type=Non-Canonical",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUMBER_USER_COMMANDS - TOTAL_NUM_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=Applied&txn-type=Canonical",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUMBER_APPLIED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=Failed&txn-type=Canonical",
      table_header: "User Commands",
      metadata: [
        TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
        TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=Applied&txn-type=Non-Canonical",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUMBER_APPLIED_USER_COMMANDS - TOTAL_NUMBER_APPLIED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=Failed&txn-type=Non-Canonical",
      table_header: "User Commands",
      metadata: [
        TOTAL_NUM_FAILED_USER_COMMANDS - TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
        TOTAL_NUM_FAILED_USER_COMMANDS - TOTAL_NUM_FAILED_CANONICAL_USER_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=Applied&txn-type=Canonical&q-type=ZKAPP",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUM_APPLIED_CANONICAL_ZKAPP_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=Applied&txn-type=Non-Canonical&q-type=ZKAPP",
      table_header: "User Commands",
      metadata: [
        row_limit,
        TOTAL_NUM_APPLIED_ZKAPP_COMMANDS - TOTAL_NUM_APPLIED_CANONICAL_ZKAPP_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=Failed&txn-type=Canonical&q-type=ZKAPP",
      table_header: "User Commands",
      metadata: [
        TOTAL_NUM_FAILED_CANONICAL_ZKAPP_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-status=Failed&txn-type=Non-Canonical&q-type=ZKAPP",
      table_header: "User Commands",
      metadata: [
        TOTAL_NUM_FAILED_ZKAPP_COMMANDS - TOTAL_NUM_FAILED_CANONICAL_ZKAPP_COMMANDS,
        TOTAL_NUMBER_USER_COMMANDS
      ]
    },
    {
      url: "/commands/user?q-token=#{MINU_TOKEN_ADDRESS}",
      table_header: "User Commands (#{MINU_SYMBOL})",
      selector: "table[data-test=user-commands-table]",
      metadata: [1, TOTAL_NUM_MINU_TOKEN_TXN, TOTAL_NUMBER_USER_COMMANDS]
    }
  ].each do |item|
    it "is correct for #{item[:url]}" do
      visit item[:url]
      wait_until_table_loaded(item[:table_header])

      # Assert row count (first metadata value)
      table_selector = item[:selector] || "table[data-test='#{to_kebab_case(remove_parentheses(item[:table_header]))}-table']"
      row_count = page.all("#{table_selector} tr:not(:has(th))", wait: 2).count
      expect(row_count).to eq(item[:metadata][0]), "Expected #{item[:metadata][0]} rows for '#{item[:table_header]}', but found #{row_count}"

      # Parse metadata using get_by_sel
      metadata_selector = "metadata-#{to_kebab_case(item[:table_header])}"
      metadata_element = get_by_sel(metadata_selector)
      metadata_text = metadata_element.text
      # Parse "x of y of z" format, removing commas for number conversion
      metadata_numbers = metadata_text.scan(/\d[\d,]*/).map { |num| num.delete(",").to_i }

      # Assert metadata values based on length
      if item[:metadata].length == 3
        # For metadata like [row_count, available, total]
        expect(metadata_numbers[1]).to eq(item[:metadata][1]), "Expected applied total of #{item[:metadata][1]} for '#{item[:table_header]}', but found #{metadata_numbers[1]}"
        expect(metadata_numbers[2]).to eq(item[:metadata][2]), "Expected overall total of #{item[:metadata][2]} for '#{item[:table_header]}', but found #{metadata_numbers[2]}"
      elsif item[:metadata].length == 2
        # For metadata like [row_count, total]
        expect(metadata_numbers[1]).to eq(item[:metadata][1]), "Expected total of #{item[:metadata][1]} for '#{item[:table_header]}', but found #{metadata_numbers[1]}"
      end
    end
  end
end
