# spec/features/tables/mina_token_accounts_spec.rb
require "spec_helper"

RSpec.describe "MINA Token Accounts table", type: :system do
  let(:url) { "/addresses/accounts" }
  let(:heading) { "MINA Token Accounts" }
  let(:columns) { ["Type", "Public Key", "Username", "Balance", "Nonce", "Delegate", "Time Locked"].map(&:upcase) }

  before(:each) do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has sortable column 'Balance'" do
    test_sortable_column(heading, "Balance", [nil, "BALANCE_ASC", "BALANCE_DESC"])
  end

  it "has working filter for column 'Type' with input 'Zkapp'" do
    test_filter(
      heading,
      "Type",
      "Zkapp",
      "select",
      lambda do
        get_table_metadata("MINA Token Accounts")
        type_cells = all(table_column_selector("MINA Token Accounts", "Type".upcase))
        type_cells.each do |cell|
          expect(cell.text).to eq("Zkapp"), "Expected 'Type' to be 'Zkapp', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Public Key' with input 'B62qpqCBExtxzfHUPkmrrfmYhXZyg3V7pSmwuxHMzTi8E6gBbopauJS'" do
    test_filter(
      heading,
      "Public Key",
      "B62qpqCBExtxzfHUPkmrrfmYhXZyg3V7pSmwuxHMzTi8E6gBbopauJS",
      nil,
      lambda do
        table_rows = get_table_rows("MINA Token Accounts")
        expect(table_rows.count).to eq(1), "Expected 'MINA Token Accounts' table to have 1 row, but found #{table_rows.count}"
        public_key_cells = all(table_column_selector("MINA Token Accounts", "Public Key".upcase))
        public_key_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq(Constants::ROMEK_ADDRESS), "Expected 'Public Key' to be '#{Constants::ROMEK_ADDRESS}', but was '#{cell.text}'"
        end
        username_cells = all(table_column_selector("MINA Token Accounts", "Username".upcase))
        username_cells.each do |cell|
          expect(cell.text).to eq(Constants::ROMEK_USERNAME), "Expected 'Username' to be '#{Constants::ROMEK_USERNAME}', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Balance' with input '5000.1234'" do
    test_filter(
      heading,
      "Balance",
      "5000.1234",
      nil,
      lambda do
        table_rows = get_table_rows("MINA Token Accounts")
        expect(table_rows.count).to eq(25), "Expected 'MINA Token Accounts' table to have 25 rows, but found #{table_rows.count}"
        balance_cells = all(table_column_selector("MINA Token Accounts", "Balance".upcase))
        balance_cells.each do |cell|
          balance = cell.text.delete(",").to_f
          expect(balance).to be <= 5000.1234, "Expected balance '#{balance}' to be <= 5000.1234"
        end
      end
    )
  end

  it "has working filter for column 'Delegate' with input 'B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9'" do
    test_filter(
      heading,
      "Delegate",
      "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9",
      nil,
      lambda do
        delegate_cells = all(table_column_selector("MINA Token Accounts", "Delegate".upcase))
        delegate_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq("B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9"), "Expected 'Delegate' to be 'B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has standard row limits" do
    # Assuming standard row limits are 10, 25, 50, 100 (common pagination options)
    select_input = get_by_sel("row-limit")
    expect(select_input.all("option").map(&:text)).to include("25", "50", "100", "250", "500", "1000"), "Expected pagination options to include 25, 50, 100, 250, 500, 1000"
  end
end
