# spec/features/tables/blocks_spec.rb
require "spec_helper"

RSpec.describe "Blocks table", type: :system do
  let(:url) { "/blocks" }
  let(:heading) { "Blocks" }
  let(:columns) { ["Height", "State Hash", "Slot", "Date", "Block Producer", "Coinbase", "User Commands", "SNARKs", "Coinbase Receiver"].map(&:upcase) }

  before do
    visit url
    page.driver.resize(1024, 2000)
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
      nil,
      lambda do
        height_cells = all(table_column_selector("Blocks", "Height".upcase))
        height_cells.each do |cell|
          height = cell.text.delete(",").to_i
          expect(height).to be <= 359900, "Expected height '#{height}' to be <= 359900"
        end
      end
    )
  end

  it "has working filter for column 'State Hash' with input '#{Constants::GENESIS_BLOCK_BLOCK_HASH}'" do
    test_filter(
      heading,
      "State Hash",
      Constants::GENESIS_BLOCK_BLOCK_HASH,
      nil,
      lambda do
        table_rows = get_table_rows("Blocks")
        expect(table_rows.count).to eq(1), "Expected 'Blocks' table to have 1 row, but found #{table_rows.count}"
        state_hash_cells = all(table_column_selector("Blocks", "State Hash".upcase))
        state_hash_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq(Constants::GENESIS_BLOCK_BLOCK_HASH), "Expected 'State Hash' to be '#{Constants::GENESIS_BLOCK_BLOCK_HASH}', but was '#{cleaned_text}'"
        end
      end
    )
  end

  it "has working filter for column 'Slot' with input '565000'" do
    test_filter(
      heading,
      "Slot",
      "565000",
      nil,
      lambda do
        slot_cells = all(table_column_selector("Blocks", "Slot".upcase))
        slot_cells.each do |cell|
          slot = cell.text.delete(",").to_i
          expect(slot).to be <= 565000, "Expected slot '#{slot}' to be <= 565000"
          expect(slot).to be > Constants::SLOTS_PER_EPOCH, "Expected slot '#{slot}' to be > #{Constants::SLOTS_PER_EPOCH}"
        end
      end
    )
  end

  it "has working filter for column 'Block Producer' with input 'B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg'" do
    test_filter(
      heading,
      "Block Producer",
      "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
      nil,
      lambda do
        block_producer_cells = all(table_column_selector("Blocks", "Block Producer".upcase))
        block_producer_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq(Constants::FIRST_BLOCK_PRODUCER_ADDRESS), "Expected 'Block Producer' to be '#{Constants::FIRST_BLOCK_PRODUCER_ADDRESS}', but was '#{cleaned_text}'"
        end
      end
    )
  end

  it "has standard row limits" do
    select_input = get_by_sel("row-limit")
    [25, 50, 100, 250, 500, 1000].each { |limit|
      select_option(select_input, limit.to_s)
      wait_until_table_loaded("Blocks")
      table_rows = get_table_rows("Blocks")
      expect(table_rows.count).to be == limit, "Expected 'Blocks' table to have row count of #{limit}, but found #{table_rows.count}"
    }
  end

  it "has working canonical filter" do
    canonical_select = find("select#canonical-selection")

    # Select "Canonical"
    canonical_select.select("Canonical")
    wait_until_table_loaded("Blocks")
    table_rows = get_table_rows("Blocks")
    table_rows.each do |row|
      expect(row).not_to have_selector(".non-canonical"), "Expected no non-canonical rows after filtering for Canonical"
      expect(row).to have_selector(".canonical"), "Expected canonical rows after filtering for Canonical"
    end

    # Select "Non-Canonical"
    canonical_select.select("Non-Canonical")
    wait_until_table_loaded("Blocks")
    table_rows = get_table_rows("Blocks")
    table_rows.each do |row|
      expect(row).to have_selector(".non-canonical"), "Expected non-canonical rows after filtering for Non-Canonical"
      expect(row).not_to have_selector(".canonical"), "Expected no canonical rows after filtering for Non-Canonical"
    end

    # Select "All"
    canonical_select.select("All")
    wait_until_table_loaded("Blocks")
    table_rows = get_table_rows("Blocks")
    expect(table_rows).to satisfy { |rows| rows.any? { |row| row.has_selector?(".non-canonical") } }, "Expected some non-canonical rows after filtering for All"
    expect(table_rows).to satisfy { |rows| rows.any? { |row| row.has_selector?(".canonical") } }, "Expected some canonical rows after filtering for All"
  end

  it "has working load next button" do
    visit "/blocks?q-height=359613"

    wait_until_table_loaded("Blocks")
    load_next_button = find("button", text: "Load Next", wait: 2, visible: false)
    load_next_button.click
    expect(load_next_button[:disabled]).to eq(true), "Expected 'Load Next' button to be disabled"
  end

  it "has user command and zk txn counts in the user command column" do
    filter_container = find("th", text: "Height".upcase)
    filter_input = filter_container.find("input")
    filter_input.set("360580")
    wait_until_table_loaded("Blocks")
    table_rows = get_table_rows("Blocks")
    first_row = table_rows.first
    expect(first_row).to have_content("58/2"), "Expected first row to contain '58/2' in the User Commands column"
  end
end
