require "spec_helper"

RSpec.describe "Accounts listing section header", type: :system do
  it "updates when clicking on the 'Mina Accounts' menu item" do
    visit "/addresses/accounts?q-token=#{Constants::MINU_TOKEN_ADDRESS}"
    wait_until_table_loaded("MINU Token Accounts")
    find("a", text: "MINA ACCOUNTS").click
    expect(page).to have_content("MINA Token Accounts")
  end
end
