# spec/features/tables/user_commands_spec.rb
require "spec_helper"

RSpec.describe "User Commands table", type: :system do
  let(:url) { "/commands/user" }
  let(:heading) { "User Commands" }
  let(:columns) { ["Height", "Txn Hash", "Date", "Type", "Status", "From", "To", "Nonce", "Fee", "Amount"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has working filter for column 'Height' with input '359613'" do
    test_filter(
      heading,
      "Height",
      "359613",
      nil,
      lambda do
        height_cells = all(table_column_selector("User Commands", "Height".upcase))
        height_cells.each do |cell|
          height = cell.text.delete(",").to_i
          expect(height).to be <= 359613, "Expected height '#{height}' to be <= 359613"
        end
      end
    )
  end

  it "has working filter for column 'Txn Hash' with input '5JvJnTKVwsxupNzXpRs5D3uQMsYSFE7NetN9o1KzbDAMjxFYziUg'" do
    test_filter(
      heading,
      "Txn Hash",
      "5JvJnTKVwsxupNzXpRs5D3uQMsYSFE7NetN9o1KzbDAMjxFYziUg",
      nil,
      lambda do
        table_rows = get_table_rows("User Commands")
        expect(table_rows.count).to eq(1), "Expected 'User Commands' table to have 1 row, but found #{table_rows.count}"
        txn_hash_cells = all(table_column_selector("User Commands", "Txn Hash".upcase))
        txn_hash_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to include("5JvJnTKVwsxupNzXpRs5D3uQMsYSFE7NetN9o1KzbDAMjxFYziUg"), "Expected 'Txn Hash' to contain '5JvJnTKVwsxupNzXpRs5D3uQMsYSFE7NetN9o1KzbDAMjxFYziUg', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Type' with input 'Zkapp'" do
    test_filter(
      heading,
      "Type",
      "Zkapp",
      "select",
      lambda do
        type_cells = all(table_column_selector("User Commands", "Type".upcase))
        type_cells.each do |cell|
          expect(cell.text).to eq("Zkapp"), "Expected 'Type' to be 'Zkapp', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Type' with input 'Payment'" do
    test_filter(
      heading,
      "Type",
      "Payment",
      "select",
      lambda do
        type_cells = all(table_column_selector("User Commands", "Type".upcase))
        type_cells.each do |cell|
          expect(cell.text).to eq("Payment"), "Expected 'Type' to be 'Payment', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Type' with input 'Stake Delegation'" do
    test_filter(
      heading,
      "Type",
      "Stake Delegation",
      "select",
      lambda do
        type_cells = all(table_column_selector("User Commands", "Type".upcase))
        type_cells.each do |cell|
          expect(cell.text).to eq("Stake Delegation"), "Expected 'Type' to be 'Stake Delegation', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Status' with input 'Applied'" do
    test_filter(
      heading,
      "Status",
      "Applied",
      "select",
      lambda do
        status_cells = all(table_column_selector("User Commands", "Status".upcase))
        status_cells.each do |cell|
          expect(cell.text).to eq("Applied"), "Expected 'Status' to be 'Applied', but was '#{cell.text}'"
        end
      end
    )
  end

  it "has working filter for column 'Status' with input 'Failed'" do
    test_filter(
      heading,
      "Status",
      "Failed",
      "select",
      lambda do
        status_cells = all(table_column_selector("User Commands", "Status".upcase))
        status_cells.each do |cell|
          expect(cell.text).to eq("Failed"), "Expected 'Status' to be 'Failed', but was '#{cell.text}'"
        end
      end
    )
  end

  it "shows Applied and Failed commands when no 'Status' filter applied" do
    test_filter(
      heading,
      "Height",
      "360252",
      nil,
      lambda do
        status_cells = all(table_column_selector("User Commands", "Status".upcase))
        status_cells.each do |cell|
          normalized_text = cell.text.strip.downcase
          expect(%w[failed applied].include?(normalized_text)).to be(true), "Expected 'Status' to be one of 'failed' or 'applied' (normalized), but was '#{normalized_text}'"
        end
      end
    )
  end

  it "has working filter for column 'From' with input '#{Constants::FIRST_SENDER_ADDRESS}'" do
    test_filter(
      heading,
      "From",
      Constants::FIRST_SENDER_ADDRESS,
      nil,
      lambda do
        from_cells = all(table_column_selector("User Commands", "From".upcase))
        from_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq(Constants::FIRST_SENDER_ADDRESS), "Expected 'From' to be '#{Constants::FIRST_SENDER_ADDRESS}', but was '#{cleaned_text}'"
        end
      end
    )
  end

  it "has working filter for column 'To' with input '#{Constants::FIRST_RECIPIENT_ADDRESS}'" do
    test_filter(
      heading,
      "To",
      Constants::FIRST_RECIPIENT_ADDRESS,
      nil,
      lambda do
        to_cells = all(table_column_selector("User Commands", "To".upcase))
        to_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq(Constants::FIRST_RECIPIENT_ADDRESS), "Expected 'To' to be '#{Constants::FIRST_RECIPIENT_ADDRESS}', but was '#{cleaned_text}'"
        end
      end
    )
  end

  it "has standard row limits" do
    select_input = get_by_sel("row-limit")
    [25, 50, 100, 250, 500, 1000].each { |limit|
      select_option(select_input, limit.to_s)
      wait_until_table_loaded("User Commands")
      table_rows = get_table_rows("User Commands")
      expect(table_rows.count).to be == limit, "Expected 'User Commands' table to have row count of #{limit}, but found #{table_rows.count}"
    }
  end

  it "has working load next button" do
    visit "/commands/user?q-height=359611&row-limit=50"
    wait_until_table_loaded("User Commands")
    load_next_button = find("button", text: "Load Next", wait: 2, visible: false)
    load_next_button.click
    expect(load_next_button[:disabled]).to eq(true), "Expected 'Load Next' button to be disabled"
  end
end
