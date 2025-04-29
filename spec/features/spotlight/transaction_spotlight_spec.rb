# spec/features/transaction_spotlight_spec.rb
require "spec_helper"

RSpec.describe "Transaction spotlight", type: :system do
  let(:devices) { ["iphone-xr", "macbook-11"] } # Define devices
  let(:mobile) { devices[0] } # iPhone XR

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
    expected_values =
      [
        ["Status:", "Applied"],
        ["Date:", "20240605 00:12:00 UTC"],
        ["Canonical:", "true"],
        ["Amount:", "1.0 MINA"],
        ["From/Fee Payer:", "B62qpjxUpgdjzwQfd8q2gzxi99wN7SCgmofpvw27MBkfNHfHoY2VH32"],
        ["Nonce:", "765"],
        ["Kind:", "Payment"],
        ["Txn Hash:", "5JuJ1eRNWdE8jSMmCDoHnAdBGhLyBnCk2gkcvkfCZ7WvrKtGuWHB"],
        ["Block Height:", "359,607"],
        ["Block State Hash:", "3NKg81uwJ61tNNbM1SkS6862AHwfRhwNQEKZemJS9UwBAzaNK8ch"],
        ["Fee:", "0.0011 MINA"],
        ["To:", "B62qpjxUpgdjzwQfd8q2gzxi99wN7SCgmofpvw27MBkfNHfHoY2VH32"]
      ]

    visit "/commands/#{Constants::FIRST_TXN_HASH}"
    test_spotlight("Command Spotlight", Constants::FIRST_TXN_HASH, expected_values)
  end

  it "displays non-canonical command" do
    expected_values = [
      ["Status:", "Applied"],
      ["Date:", "20240610 02:27:00 UTC"],
      ["Txn Hash:", "5JurAvgK6MAjZ9EMsV11dxQTef7TX5KLKnJzwRXEtD7HUoscww38"],
      ["Block Height:", "360,998"],
      ["Canonical:", "false"],
      ["Block State Hash:", "3NLnBeReHAkWkUeeUeFzHjEXb7UamxyKmhGcycFzyjisE2nWRmak"],
      ["Amount:", "0.08 MINA"],
      ["Fee:", "0.01 MINA"],
      ["From/Fee Payer:", "B62qnEeb4KAp9WxdMxddHVtJ8gwfyJURG5BZZ6e4LsRjQKHNWqmgSWt"],
      ["To:", "B62qq6PqndihT5uoGAXzndoNgYSUMvUPmVqMQATusaoS1ZmCZRcM1ku"],
      ["Nonce:", "243,120"],
      ["Memo:", ""],
      ["Kind:", "Payment"],
      ["Block Height:", "360,998"],
      ["Block State Hash:", "3NLnBeReHAkWkUeeUeFzHjEXb7UamxyKmhGcycFzyjisE2nWRmak"]
    ]
    visit "/commands/#{Constants::FIRST_NON_CANONICAL_TXN_HASH}"
    test_spotlight("Command Spotlight", Constants::FIRST_NON_CANONICAL_TXN_HASH, expected_values)
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

    expected_values = ["-1.0", "0.0", "0.0", "-19.0", "19.0", "0.0", "1.0"]
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
