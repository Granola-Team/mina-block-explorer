# spec/features/transaction_spotlight_spec.rb
require "spec_helper"

RSpec.describe "Transaction spotlight", type: :system do
  let(:devices) { ["iphone-xr", "macbook-11"] } # Define devices
  let(:mobile) { devices[0] } # iPhone XR
  let(:expected_fields) do
    [
      "Status",
      "Date",
      "Canonical",
      "Amount",
      "From/Fee Payer",
      "Nonce",
      "Kind",
      "Txn Hash",
      "Block Height",
      "Block State Hash",
      "Fee",
      "To",
      "Memo"
    ]
  end

  it "displays proper status" do
    # Test Failed status
    visit "/commands/#{Constants::FAILED_TXN_HASH}"
    wait_until_spotlight_loaded
    within("section#spotlight-section table") do
      status_row = find("tr", text: "Status")
      status_value = status_row.find("td", match: :first)
      expect(status_value.text).to eq("Failed"), "Expected Status to be 'Failed', but was '#{status_value.text}'"
    end

    # Test Applied status
    visit "/commands/#{Constants::APPLIED_TXN_HASH}?q-state-hash=#{Constants::APPLIED_TXN_BLOCK_STATE_HASH}"
    wait_until_spotlight_loaded
    within("section#spotlight-section table") do
      status_row = find("tr", text: "Status")
      status_value = status_row.find("td", match: :first)
      expect(status_value.text).to eq("Applied"), "Expected Status to be 'Applied', but was '#{status_value.text}'"
    end
  end

  it "displays complete information" do
    # Set viewport to iPhone XR (414x896)
    page.driver.resize(414, 896)

    visit "/commands/#{Constants::FIRST_TXN_HASH}"
    test_spotlight("Command Spotlight", Constants::FIRST_TXN_HASH, expected_fields)
  end

  it "displays non-canonical command" do
    visit "/commands/#{Constants::FIRST_NON_CANONICAL_TXN_HASH}"
    test_spotlight("Command Spotlight", Constants::FIRST_NON_CANONICAL_TXN_HASH, expected_fields)
  end

  it "renders the tooltip for stake delegations" do
    visit "/commands/#{Constants::STAKE_DELEGATION_HASH}"
    wait_until_spotlight_loaded
    within("section#spotlight-section table") do
      amount_row = find("tr", text: "Amount")
      tooltip = amount_row.find("td .tooltip", match: :first)
      expect(tooltip[:title]).to eq("Stake delegations have no transacted amount"), "Expected tooltip title to be 'Stake delegations have no transacted amount', but was '#{tooltip[:title]}'"
    end
  end

  it "does not render the tooltip for regular payments" do
    visit "/commands/#{Constants::FIRST_TXN_HASH}"
    wait_until_spotlight_loaded
    within("section#spotlight-section table") do
      amount_row = find("tr", text: "Amount")
      expect(amount_row).not_to have_selector("td .tooltip"), "Expected no tooltip in Amount row for regular payment"
    end
  end

  it "displays other blocks containing the same txn" do
    visit "/commands/#{Constants::TXN_HASH_IN_OTHER_BLOCKS}"
    wait_until_spotlight_loaded
    expect(page).to have_content("In Other Blocks"), "Expected 'In Other Blocks' section to be present"

    table_rows = get_table_rows("In Other Blocks")
    expect(table_rows.count).to eq(1), "Expected 'In Other Blocks' table to have 1 row, but found #{table_rows.count}"

    # Verify table headers
    table = find("table[data-test='#{to_kebab_case("In Other Blocks")}-table']")
    header_row = table.find("tr:has(th)")
    headers = header_row.all("th").map { |th| th.text.strip }
    expected_headers = ["Height".upcase, "Block State Hash".upcase]
    expect(headers).to eq(expected_headers), "Expected 'In Other Blocks' table headers to be #{expected_headers}, but found #{headers}"
  end

  it "displays zk app sections for zk app txn" do
    visit "/commands/#{Constants::ZK_APP_TXN_HASH}"

    # Verify "Accounts Updated" table
    wait_until_table_loaded("Accounts Updated")
    table_rows = get_table_rows("Accounts Updated")
    expect(table_rows.count).to eq(7), "Expected 'Accounts Updated' table to have 7 rows, but found #{table_rows.count}"

    expected_values = [-1, 0, 0, -19, 19, 0, 1]
    balance_cells = all(table_column_selector("Accounts Updated", "Balance Change".upcase))
    balance_cells.each do |cell|
      next_val = expected_values.shift
      expect(cell.text).to eq(next_val.to_s), "Expected 'Balance Change' to be '#{next_val}', but found '#{cell.text}'"
    end

    table_rows = get_table_rows("Actions Events", transposed: true)
    expect(table_rows.count).to eq(2), "Expected 'Actions & Events' table to have 2 rows (transposed), but found #{table_rows.count}"
  end

  it "should not display zk app sections for standard txn" do
    visit "/commands/#{Constants::FIRST_TXN_HASH}"
    wait_until_spotlight_loaded
    expect(page).not_to have_content("Accounts Updated"), "Expected 'Accounts Updated' section to not be present"
    expect(page).not_to have_content("Actions & Events"), "Expected 'Actions & Events' section to not be present"
  end
end
