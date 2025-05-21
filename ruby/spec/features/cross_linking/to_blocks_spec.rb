require "spec_helper"

RSpec.describe "Block page navigation", type: :system do
  pages = [
    {
      origin: "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}/#{Constants::FIRST_BLOCK_PRODUCER_ADDRESS}/block-production",
      dest: "blocks",
      href: "/blocks?q-block-producer=#{Constants::FIRST_BLOCK_PRODUCER_ADDRESS}"
    }
  ]

  pages.each do |item|
    it "is navigated to from #{item[:origin]}" do
      visit item[:origin]

      wait_until_table_loaded("Block Production")

      # Click the link with text "See all block production"
      find("a", text: "SEE ALL BLOCK PRODUCTION").click

      # Verify the URL contains the expected href
      expect(page.current_url).to include(item[:href]), "Expected URL to contain '#{item[:href]}', but was #{page.current_url}"

      # Verify the "Block Producer" column in the "Blocks" table contains FIRST_BLOCK_PRODUCER_ADDRESS
      wait_until_table_loaded("Blocks")
      producer_cells = all(table_column_selector("Blocks", "BLOCK PRODUCER"))
      producer_cells.each do |cell|
        cleaned_text = cell.text.gsub(/[\n+-]/, "")
        expect(cleaned_text).to include(Constants::FIRST_BLOCK_PRODUCER_ADDRESS), "Expected 'Block Producer' column to contain '#{Constants::FIRST_BLOCK_PRODUCER_ADDRESS}', but found '#{cleaned_text}'"
      end
    end
  end

  it "is navigated to from nav menu" do
    visit "/tokens"
    wait_until_table_loaded("Tokens")
    click_nav_menu_item(["Blocks"])
    expect(page.current_path).to match(/\/blocks/), "Expected URL to include '/blocks', but was #{page.current_path}"
  end
end
