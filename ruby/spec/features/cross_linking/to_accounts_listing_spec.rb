require "spec_helper"

RSpec.describe "Accounts listing page navigation", type: :system do
  pages = [
    {
      origin: "/tokens",
      column: "Holders",
      tableHeader: "Tokens"
    }
  ]

  pages.each do |item|
    it "is navigated to from #{item[:origin]} by clicking link in '#{item[:column]}'" do
      visit item[:origin]
      wait_until_table_loaded(item[:tableHeader])
      click_link_in_table_column(item[:tableHeader], item[:column].upcase, 1)
      expect(page.current_path).to match(/\/addresses\/accounts/), "Expected URL to include '/addresses/accounts', but was #{page.current_path}"
    end
  end

  it "is navigated to from nav menu" do
    visit "/"
    wait_until_table_loaded("Blocks")
    click_nav_menu_item(["Accounts"])
    expect(page.current_path).to match(/\/addresses\/accounts/), "Expected URL to include '/addresses/accounts', but was #{page.current_path}"
  end
end
