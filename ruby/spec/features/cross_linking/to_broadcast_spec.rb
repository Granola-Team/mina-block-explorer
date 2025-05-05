require "spec_helper"

RSpec.describe "Broadcast page", type: :system do
  tabs = [
    {text: "TRANSACTION", heading: "Broadcast Signed Transaction"},
    {text: "DELEGATION", heading: "Broadcast Signed Delegation"},
    {text: "LEDGER", heading: "Broadcast Signed Transaction From Ledger"}
  ]

  it "contains a tab menu with #{tabs.length} tabs" do
    visit "/broadcast/transaction"

    # Verify the number of tabs in the menu
    tab_links = find_all("menu#tabs li a")
    expect(tab_links.count).to eq(tabs.length), "Expected #{tabs.length} tabs, but found #{tab_links.count}"

    # Iterate through each tab and verify its heading, textarea, and submit button
    tabs.each do |tab|
      # Click the tab link with the specified text
      find("menu#tabs li a", text: tab[:text]).click

      # Verify the heading exists
      expect(page).to have_content(tab[:heading]), "Expected heading '#{tab[:heading]}' to exist"

      # Verify the textarea exists
      expect(page).to have_selector("section form textarea"), "Expected a textarea in the form"

      # Verify the submit button exists
      expect(page).to have_selector("section form input[type='submit']"), "Expected a submit button in the form"
    end
  end

  it "is navigated to from nav menu" do
    visit "/"
    wait_until_table_loaded("Blocks")
    click_nav_menu_item(["More", "Send"])
    expect(page.current_path).to match(/\/broadcast\/transaction/), "Expected URL to include '/broadcast/transaction', but was #{page.current_path}"
  end
end
