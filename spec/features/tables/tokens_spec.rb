# spec/features/tables/tokens_spec.rb
require "spec_helper"

RSpec.describe "Tokens table", type: :system do
  let(:url) { "/tokens" }
  let(:heading) { "Tokens" }
  let(:columns) { ["Symbol", "Supply", "ID", "Owner", "Holders", "Transactions", "% Unlocked"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has working filter for column 'Symbol' with input 'MINU'" do
    test_filter(
      heading,
      "Symbol",
      "MINU",
      nil,
      lambda do
        table_rows = get_table_rows("Tokens")
        expect(table_rows.count).to eq(1), "Expected 'Tokens' table to have 1 row, but found #{table_rows.count}"
        symbol_cells = all(table_column_selector("Tokens", "Symbol".upcase))
        symbol_cells.each do |cell|
          expect(cell.text).to eq("MINU"), "Expected 'Symbol' to be 'MINU', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'ID' with input '#{Constants::MINU_TOKEN_ADDRESS}'" do
    test_filter(
      heading,
      "ID",
      Constants::MINU_TOKEN_ADDRESS,
      nil,
      lambda do
        table_rows = get_table_rows("Tokens")
        expect(table_rows.count).to eq(1), "Expected 'Tokens' table to have 1 row, but found #{table_rows.count}"
        id_cells = all(table_column_selector("Tokens", "ID".upcase))
        id_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq(Constants::MINU_TOKEN_ADDRESS), "Expected 'ID' to be '#{Constants::MINU_TOKEN_ADDRESS}', but was '#{cleaned_text}'"
        end
      end
    )
  end
end
