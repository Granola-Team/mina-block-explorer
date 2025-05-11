# spec/features/tables/account_spotlight_spec.rb
require "spec_helper"

RSpec.describe "Internal Commands table", type: :system do
  let(:url) { "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/commands/internal" }
  let(:heading) { "Internal Commands" }
  let(:columns) { ["Height", "State Hash", "Fee", "Type", "Date"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has working filter for column 'Height' with input '359900'" do
    test_filter(
      heading,
      "Height",
      "359900",
      nil, # filter_type is not specified, defaults to input
      lambda do
        sleep 1 # Equivalent to cy.wait(1000)
        height_cells = all(table_column_selector("Internal Commands", "Height".upcase))
        height_cells.each do |cell|
          height = cell.text.delete(",").to_i
          expect(height).to be <= 359900, "Expected height '#{height}' to be <= 359900"
        end
      end
    )
  end

  it "has working filter for column 'State Hash' with input '3NKgJBsyECQga3PSKvJRSWq1we8GgE4gawMTZv4eH6ebk8ZTxL34'" do
    test_filter(
      heading,
      "State Hash",
      "3NKgJBsyECQga3PSKvJRSWq1we8GgE4gawMTZv4eH6ebk8ZTxL34",
      nil, # filter_type is not specified, defaults to input
      lambda do
        table_rows = get_table_rows("Internal Commands")
        expect(table_rows).not_to be_empty, "Expected 'Internal Commands' table to have rows after filtering"
        state_hash_cells = all(table_column_selector("Internal Commands", "State Hash".upcase))
        state_hash_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to include("3NKgJBsyECQga3PSKvJRSWq1we8GgE4gawMTZv4eH6ebk8ZTxL34"), "Expected 'State Hash' to contain '3NKgJBsyECQga3PSKvJRSWq1we8GgE4gawMTZv4eH6ebk8ZTxL34', but was '#{cell.text}'"
        end
      end
    )
  end

  it "links to internal commands with correct filter" do
    find("a", text: "See all internal commands".upcase).click
    expect(page.current_url).to include("/commands/internal?q-recipient=#{Constants::GENESIS_ACCOUNT_PK}"), "Expected URL to include '/commands/internal?q-recipient=#{Constants::GENESIS_ACCOUNT_PK}', but was '#{page.current_url}'"

    wait_until_table_loaded("Internal Commands")
    recipient_cells = all(table_column_selector("Internal Commands", "Recipient".upcase))
    recipient_cells.each do |cell|
      cleaned_text = cell.text.gsub(/[\n+-]/, "")
      expect(cleaned_text).to include(Constants::GENESIS_ACCOUNT_PK), "Expected 'Recipient' column to contain '#{Constants::GENESIS_ACCOUNT_PK}', but found '#{cleaned_text}'"
    end
  end
end

RSpec.describe "Delegations table", type: :system do
  let(:url) { "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/delegations" }
  let(:heading) { "Delegations" }
  let(:columns) { ["Account", "Delegated Balance", "% of Delegation"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end
end
