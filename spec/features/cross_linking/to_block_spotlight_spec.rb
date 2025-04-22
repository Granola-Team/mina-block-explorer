require "spec_helper"

RSpec.describe "Block spotlight navigation", type: :system do
  pages = [
    {
      origin: "/commands/internal",
      column: "STATE HASH",
      tableHeader: "Internal Commands"
    },
    {
      origin: "/blocks",
      column: "STATE HASH",
      tableHeader: "Blocks"
    },
    {
      origin: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/block-production",
      column: "STATE HASH",
      tableHeader: "Block Production"
    }
  ]

  pages.each do |item|
    it "is navigated to from #{item[:origin]}" do
      visit item[:origin]
      wait_until_table_loaded(item[:tableHeader], wait: 10)
      click_link_in_table_column(item[:tableHeader], item[:column].upcase, 1)
      expect(page.current_path).to match(/\/blocks\//), "Expected URL to include '/blocks/', but was #{page.current_path}"
    end
  end
end
