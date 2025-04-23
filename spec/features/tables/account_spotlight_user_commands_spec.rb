# spec/features/tables/user_commands_spec.rb
require "spec_helper"

RSpec.describe "User Commands table", type: :system do
  let(:url) { "/addresses/accounts/#{Constants::FIRST_SENDER_ADDRESS}/commands/user" }
  let(:heading) { "User Commands" }
  let(:columns) { ["Height", "Txn Hash", "Nonce", "Date", "Type", "Direction", "Counterparty", "Amount/Fee"].map(&:upcase) }

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
        height_cells = all(table_column_selector("User Commands", "Height".upcase), wait: 0)
        height_cells.each do |cell|
          height = cell.text.delete(",").to_i
          expect(height).to be <= 359900, "Expected height '#{height}' to be <= 359900"
        end
      end
    )
  end

  it "has working filter for column 'Txn Hash' with input '5JuqQth67hX432bpfrkpcA5ceayBQt8dBLZxYRnPbWBmLasryP3b'" do
    test_filter(
      heading,
      "Txn Hash",
      "5JuqQth67hX432bpfrkpcA5ceayBQt8dBLZxYRnPbWBmLasryP3b",
      nil, # filter_type is not specified, defaults to input
      lambda do
        table_rows = get_table_rows("User Commands")
        expect(table_rows).not_to be_empty, "Expected 'User Commands' table to have rows after filtering"
        txn_hash_cells = all(table_column_selector("User Commands", "Txn Hash".upcase), wait: 1)
        txn_hash_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to include("5JuqQth67hX432bpfrkpcA5ceayBQt8dBLZxYRnPbWBmLasryP3b"), "Expected 'Txn Hash' to contain '5JuqQth67hX432bpfrkpcA5ceayBQt8dBLZxYRnPbWBmLasryP3b', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Type' with input 'Zkapp'" do
    test_filter(
      heading,
      "Type".upcase,
      "Zkapp",
      "select",
      lambda do
        type_cells = all(table_column_selector("User Commands", "Type".upcase), wait: 1)
        type_cells.each do |cell|
          expect(cell.text).to eq("Zkapp"), "Expected 'Type' to be 'Zkapp', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Direction' with input 'In'" do
    test_filter(
      heading,
      "Direction".upcase,
      "In",
      "select",
      lambda do
        direction_cells = all(table_column_selector("User Commands", "Direction".upcase))
        direction_cells.each do |cell|
          expect(cell.text).to eq("IN"), "Expected 'Direction' to be 'In', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Direction' with input 'Out'" do
    test_filter(
      heading,
      "Direction".upcase,
      "Out",
      "select",
      lambda do
        direction_cells = all(table_column_selector("User Commands", "Direction".upcase))
        direction_cells.each do |cell|
          expect(cell.text).to eq("OUT"), "Expected 'Direction' to be 'Out', but was '#{cell.text}'"
        end
      end
    )
  end
end
