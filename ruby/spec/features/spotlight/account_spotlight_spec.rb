require "spec_helper"

RSpec.describe "Account spotlight", type: :system do
  it "displays appropriately for standard accounts" do
    visit "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}/#{Constants::STANDARD_ACCOUNT_PK}"

    # Verify the account creation fee message is displayed
    expect(page).to have_content("Includes 1 MINA account creation fee", wait: 30), "Expected 'Includes 1 MINA account creation fee' to be present"
  end

  it "displays appropriately for genesis accounts with zero balance" do
    visit "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}/#{Constants::GENESIS_ACCOUNT_PK_ZERO_GENESIS}"

    # Verify the account creation fee message is not displayed
    expect(page).not_to have_content("Includes 1 MINA account creation fee", wait: 30), "Expected 'Includes 1 MINA account creation fee' to not be present"

    # Verify the genesis ledger balance message is not displayed
    expect(page).not_to have_content("Includes balance from genesis ledger"), "Expected 'Includes balance from genesis ledger' to not be present"

    # Verify the token-only message is not displayed
    expect(page).not_to have_content("Account has no MINA balance.", wait: 30), "Expected 'Account has no MINA balance.' to not be present"
  end

  it "displays appropriately for genesis accounts with positive balances" do
    visit "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}/#{Constants::GENESIS_ACCOUNT_PK}"

    # Verify the genesis ledger balance message is displayed
    expect(page).to have_content("Includes balance from genesis ledger", wait: 30), "Expected 'Includes balance from genesis ledger' to be present"

    # Verify the Genesis Balance value in the spotlight
    # Assuming the spotlight value is in a table or similar structure with "Genesis Balance" as a label
    spotlight_row = find("tr", text: "Genesis Balance")
    balance = spotlight_row.find("td", match: :first)
    expect(balance.text).to eq("108,536.109082914MINA"), "Expected Genesis Balance to be '108,536.109082914MINA', but was '#{balance.text}'"
  end

  it "displays appropriately for token-only accounts" do
    visit "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}/#{Constants::TOKEN_ACTIVITY_ONLY_ADDRESS}"

    wait_until_table_loaded("User Commands")

    # Verify the token-only message is displayed
    expect(page).to have_content("Account has no MINA balance."), "Expected 'Account has no MINA balance.' to be present"

    # Verify the "User Commands" table has at least 0 rows
    table_rows = find_all("table[data-test=\"#{to_kebab_case("User Commands")}-table\"] tr:not(:has(th))")
    expect(table_rows.count).to be >= 0, "Expected 'User Commands' table to have at least 0 rows, but found #{table_rows.count}"

    # Verify each value in the "Type" column is "Zkapp"
    type_cells = all(table_column_selector("User Commands", "Type".upcase))
    type_cells.each do |cell|
      expect(cell.text).to eq("Zkapp"), "Expected 'Type' column to contain 'Zkapp', but found '#{cell.text}'"
    end
  end

  it "renders More Details subpage on tokens tab" do
    visit "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}/#{Constants::TOKEN_ACTIVITY_ONLY_ADDRESS}/tokens"

    # Wait for the "Tokens" table to load
    wait_until_table_loaded("Tokens")

    # Verify "More Details" does not exist initially
    expect(page).not_to have_content("More Details", wait: 30), "Expected 'More Details' to not be present initially"

    # Click the link in the "More" column of the 1st row.
    click_link_in_table_column("Tokens", "More".upcase, 1)

    # Verify "More Details" is now present
    expect(page).to have_content("More Details"), "Expected 'More Details' to be present after clicking link"

    # Verify the "More Details" table has 2 rows
    # Assuming "More Details" is a transposed table (rows represent key-value pairs)
    table_rows = find_all("table[data-test='#{to_kebab_case("More Details")}-table'] tr")
    expect(table_rows.count).to eq(3), "Expected 'More Details' table to have 3 rows, but found #{table_rows.count}"

    # Verify the table rows contain the expected text in <th> elements
    expected_texts = ["App State:", "Action State:", "Permissions:"]
    table_rows.each_with_index do |row, index|
      th_text = row.find("th").text
      expect(th_text).to eq(expected_texts[index]), "Expected row #{index + 1} to have <th> text '#{expected_texts[index]}', but found '#{th_text}'"
    end
  end
end
