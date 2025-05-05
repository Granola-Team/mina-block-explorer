# spec/features/tables/snarks_spec.rb
require "spec_helper"

RSpec.describe "SNARKs table", type: :system do
  let(:url) { "/snarks" }
  let(:heading) { "SNARKs" }
  let(:columns) { ["Height", "State Hash", "Date", "Prover", "Fee"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has working filter for column 'Height' with input '360100'" do
    test_filter(
      heading,
      "Height",
      "360100",
      nil,
      lambda do
        metadata = get_table_metadata("SNARKs".downcase)
        expect(metadata.length).to eq(2), "Expected 'SNARKs' table metadata to have 2 datum, but found #{metadata.length}"
        height_cells = all(table_column_selector("SNARKs", "Height".upcase))
        height_cells.each do |cell|
          height = cell.text.delete(",").to_i
          expect(height).to be <= 360100, "Expected height '#{height}' to be <= 360100"
        end
      end
    )
  end

  it "has working filter for column 'State Hash' with input '#{Constants::BLOCK_WITH_ALL_ACTIVITY}'" do
    test_filter(
      heading,
      "State Hash",
      Constants::BLOCK_WITH_ALL_ACTIVITY,
      nil,
      lambda do
        metadata = get_table_metadata("SNARKs".downcase)
        expect(metadata.length).to eq(2), "Expected 'SNARKs' table metadata to have 2 datum, but found #{metadata.length}"
        table_rows = get_table_rows("SNARKs".downcase)
        expect(table_rows.count).to be > 1, "Expected 'SNARKs' table to have more than 1 row, but found #{table_rows.count}"
        state_hash_cells = all(table_column_selector("SNARKs".downcase, "State Hash".upcase))
        state_hash_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq(Constants::BLOCK_WITH_ALL_ACTIVITY), "Expected 'State Hash' to be '#{Constants::BLOCK_WITH_ALL_ACTIVITY}', but was '#{cleaned_text}'"
        end
      end
    )
  end

  it "has working filter for column 'Prover' with input 'B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC'" do
    test_filter(
      heading,
      "Prover",
      "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC",
      nil,
      lambda do
        metadata = get_table_metadata("SNARKs".downcase)
        expect(metadata.length).to eq(2), "Expected 'SNARKs' table metadata to have 2 datum, but found #{metadata.length}"
        prover_cells = all(table_column_selector("SNARKs".downcase, "Prover".upcase))
        prover_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq("B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC"), "Expected 'Prover' to be 'B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC', but was '#{cleaned_text}'"
        end
      end
    )
  end

  it "has standard row limits" do
    select_input = get_by_sel("row-limit")
    [25, 50, 100, 250, 500, 1000].each { |limit|
      select_option(select_input, limit.to_s)
      wait_until_table_loaded("SNARKs")
      table_rows = get_table_rows("SNARKs")
      expect(table_rows.count).to be == limit, "Expected 'SNARKs' table to have row count of #{limit}, but found #{table_rows.count}"
    }
  end

  it "has working load next button" do
    visit "/snarks?row-limit=50&q-height=359630"
    wait_until_table_loaded("SNARKs".downcase)
    load_next_button = find("button", text: "Load Next", wait: 2, visible: false)
    load_next_button.click
    expect(load_next_button[:disabled]).to eq(true), "Expected 'Load Next' button to be disabled"
  end
end
