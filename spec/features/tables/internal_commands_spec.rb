# spec/features/tables/internal_commands_spec.rb
require "spec_helper"

RSpec.describe "Internal Commands table", type: :system do
  let(:url) { "/commands/internal" }
  let(:heading) { "Internal Commands" }
  let(:columns) { ["Height", "State Hash", "Recipient", "Fee", "Type", "Date"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has working filter for column 'Height' with input '359610'" do
    test_filter(
      heading,
      "Height",
      "359610",
      nil,
      lambda do
        metadata = get_table_metadata("Internal Commands")
        expect(metadata.length).to eq(2), "Expected 'Internal Commands' table metadata to have 2 datum, but found #{metadata.length}"
        height_cells = all(table_column_selector("Internal Commands", "Height".upcase))
        height_cells.each do |cell|
          height = cell.text.delete(",").to_i
          expect(height).to be <= 359610, "Expected height '#{height}' to be <= 359610"
        end
      end
    )
  end

  it "has working filter for column 'State Hash' with input '3NLCZXPQH8WwSXUtrCHHz3n78RQJxNYUyn4uyx6JyTo4THApAbZd'" do
    test_filter(
      heading,
      "State Hash",
      "3NLCZXPQH8WwSXUtrCHHz3n78RQJxNYUyn4uyx6JyTo4THApAbZd",
      nil,
      lambda do
        metadata = get_table_metadata("Internal Commands")
        expect(metadata.length).to eq(2), "Expected 'Internal Commands' table metadata to have 2 datum, but found #{metadata.length}"
        table_rows = get_table_rows("Internal Commands")
        expect(table_rows.count).to be > 1, "Expected 'Internal Commands' table to have more than 1 row, but found #{table_rows.count}"
        state_hash_cells = all(table_column_selector("Internal Commands", "State Hash".upcase))
        state_hash_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq("3NLCZXPQH8WwSXUtrCHHz3n78RQJxNYUyn4uyx6JyTo4THApAbZd"), "Expected 'State Hash' to be '3NLCZXPQH8WwSXUtrCHHz3n78RQJxNYUyn4uyx6JyTo4THApAbZd', but was '#{cleaned_text}'"
        end
      end
    )
  end

  it "has working filter for column 'Recipient' with input 'B62qioAD9geuKsffk9gXSgHf18riNEB9NmR4Zyuo2fvWd5WWYTg4WHB'" do
    test_filter(
      heading,
      "Recipient",
      "B62qioAD9geuKsffk9gXSgHf18riNEB9NmR4Zyuo2fvWd5WWYTg4WHB",
      nil,
      lambda do
        metadata = get_table_metadata("Internal Commands")
        expect(metadata.length).to eq(2), "Expected 'Internal Commands' table metadata to have 2 datum, but found #{metadata.length}"
        recipient_cells = all(table_column_selector("Internal Commands", "Recipient".upcase))
        recipient_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq("B62qioAD9geuKsffk9gXSgHf18riNEB9NmR4Zyuo2fvWd5WWYTg4WHB"), "Expected 'Recipient' to be 'B62qioAD9geuKsffk9gXSgHf18riNEB9NmR4Zyuo2fvWd5WWYTg4WHB', but was '#{cleaned_text}'"
        end
      end
    )
  end

  it "has standard row limits" do
    select_input = get_by_sel("row-limit")
    expect(select_input.all("option").map(&:text)).to include("25", "50", "100", "250", "500", "1000"), "Expected pagination options to include 25, 50, 100, 250, 500, 1000"
  end

  it "has working load next button" do
    visit "/commands/internal?q-height=359618"
    wait_until_table_loaded("Internal Commands", wait: 2)
    load_next_button = find("button", text: "Load Next", wait: 2, visible: false)
    load_next_button.click
    expect(load_next_button[:disabled]).to eq(true), "Expected 'Load Next' button to be disabled"
  end
end
