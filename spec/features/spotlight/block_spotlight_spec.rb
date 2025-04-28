# spec/features/block_spotlight_spec.rb
require "spec_helper"

RSpec.describe "Block spotlight", type: :system do
  let(:expected_values) do
    [
      ["State Hash:", "3NLjRmTyUzeA7meRAT3Yjqxzfe95GKBgkLPD2iLeVE5RMCFcw8eL"],
      ["Previous State Hash:", "3NKRe6U3WWEPX2dvuwfCLVX1nw8v5BWU1CZey9Wb9yhoeeHYDXXw"],
      ["Staged Ledger Hash:", "jwe76qJhJNSqBDxE1ZAwnT7NLzJhznUmZRPpfuaHCWF6sQvtAq7"],
      ["Snarked Ledger Hash:", "jwNw4qb6tnNhpQNxiMLem9WumxZTwmbSx3fYXW4FP3hZRkoQJSE"],
      ["Coinbase:", "720.0MINA"],
      ["Coinbase Receiver:", "B62qk2EqDADtDejTwGFi7Fp1p6ft34oi8ExnE8TmHfoqb71cotL6Mfv"],
      ["SNARK Fees:", "0.0MINA"],
      ["Global Slot:", "564,524"],
      ["Slot:", "464"],
      ["Epoch:", "0"],
      ["Transaction Fees:", "1.4161MINA"],
      ["Blockchain Length:", "359,630"],
      ["Canonical:", "true"],
      ["Total MINA:", "1,166,062,074.840039168MINA"]
    ]
  end

  def test_for_completeness(state_hash)
    # Test on the spotlight subpage
    visit "/blocks/#{state_hash}/spotlight"
    test_spotlight("Block Spotlight", state_hash, expected_values)

    # Navigate to "User Commands" tab and verify rows
    find("a#{tab_selector("User Commands")}", match: :first).click
    wait_until_table_loaded("User Commands")
    table_rows = get_table_rows("User Commands")
    expect(table_rows.count).to be > 0, "Expected 'User Commands' table to have more than 0 rows, but found #{table_rows.count}"

    # Navigate to "SNARK Jobs" tab and verify rows and Hash column
    find("a#{tab_selector("SNARK Jobs")}", match: :first).click
    wait_until_table_loaded("SNARK Jobs")
    table_rows = get_table_rows("SNARK Jobs")
    expect(table_rows.count).to be > 0, "Expected 'SNARK Jobs' table to have more than 0 rows, but found #{table_rows.count}"
    hash_cells = all(table_column_selector("SNARK Jobs", "State Hash".upcase))
    hash_cells.each do |cell|
      cleaned_text = cell.text.gsub(/[\n+-]/, "")
      expect(cleaned_text).to eq(state_hash), "Expected 'Hash' column to contain '#{state_hash}', but found '#{cell.text}'"
    end

    # Navigate to "Internal Commands" tab and verify rows
    find("a#{tab_selector("Internal Commands")}", match: :first).click
    wait_until_table_loaded("Internal Commands")
    table_rows = get_table_rows("Internal Commands")
    expect(table_rows.count).to be > 0, "Expected 'Internal Commands' table to have more than 0 rows, but found #{table_rows.count}"
  end

  it "displays complete information for canonical block" do
    test_for_completeness(Constants::FIRST_BLOCK_WITH_SNARK_WORK)
  end
end
