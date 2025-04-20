require "spec_helper"

RSpec.describe "Input", type: :system do
  slow_input_searches = [
    {heading: "MINA Accounts", url: "/addresses/accounts", input: "B62", column: "Public Key"},
    {heading: "Blocks", url: "/blocks", input: "253134", column: "Height"},
    {heading: "Staking Ledger - Epoch 1", url: "/staking-ledgers?epoch=1", input: "B62", column: "Key"},
    {heading: "Internal Commands", url: "/commands/internal", input: "253134", column: "Height"}
  ]

  slow_input_searches.each do |search|
    it "remains focused as user types slowly into #{search[:column]} on page #{search[:url]}" do
      visit search[:url]
      wait_until_table_loaded(search[:heading])
      css_selector = "#q-#{to_kebab_case(search[:column])}"
      input_field = find(css_selector)
      search[:input].chars.each do |char|
        input_field.send_keys(char)
        sleep 0.75 # 750ms delay per character
      end
      expect(page).to have_css("#{css_selector}:focus", wait: 5)
    end
  end
end
