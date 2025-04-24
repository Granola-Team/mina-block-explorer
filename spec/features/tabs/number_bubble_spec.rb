require "spec_helper"

RSpec.describe "Number bubble in tab", type: :system do
  tabs = [
    # {
    #   url: "/blocks/#{Constants::BLOCK_WITH_ALL_ACTIVITY}/commands/user",
    #   tab: "User Commands",
    #   comparison_method: "rows"
    # },
    # {
    #   url: "/blocks/#{Constants::BLOCK_WITH_ALL_ACTIVITY}/snark-jobs",
    #   tab: "SNARK Jobs",
    #   comparison_method: "rows"
    # },
    # {
    #   url: "/blocks/#{Constants::BLOCK_WITH_ALL_ACTIVITY}/commands/internal",
    #   tab: "Internal Commands",
    #   comparison_method: "rows"
    # },
    # {
    #   url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/commands/user",
    #   tab: "User Commands"
    # },
    {
      url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/block-production",
      tab: "Block Production"
    }
    # {
    #   url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/delegations",
    #   tab: "Delegations"
    # },
    ## {
    ##   url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/tokens",
    ##   tab: "Tokens"
    ## },
    # {
    #   url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/snark-jobs",
    #   tab: "SNARK Jobs"
    # },
    # {
    #   url: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/commands/internal",
    #   tab: "Internal Commands"
    # }
  ]

  tabs.each do |test_case|
    it "matches row count on tab '#{test_case[:tab]}' at #{test_case[:url]}" do
      visit test_case[:url]

      # Wait for the table associated with the tab to load
      wait_until_table_loaded(test_case[:tab])

      bubble = find("#{tab_selector(test_case[:tab])} .number-bubble")
      number = bubble.text.to_i

      if test_case[:comparison_method] == "rows"
        table_rows = find_all("table[data-test='#{to_kebab_case(test_case[:tab])}-table'] tr:not(:has(th))")
        expect(table_rows.count).to eq(number), "Expected '#{test_case[:tab]}' table to have #{number} rows, but found #{table_rows.count}"
      else
        metadata_numbers = get_table_metadata(test_case[:tab])
        expect(metadata_numbers[1]).to eq(number), "Expected '#{test_case[:tab]}' metadata to have value #{number}, but found #{metadata_numbers[1]}"
      end
    end
  end
end
