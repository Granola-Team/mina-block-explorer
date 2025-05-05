require "spec_helper"

RSpec.describe "Accounts listing section header", type: :system do
  it "updates when clicking on the 'Mina Accounts' menu item" do
    visit "/addresses/accounts?q-token=#{Constants::MINU_TOKEN_ADDRESS}"
    expect(page).to have_content("MINU Token Accounts")
    find("a", text: "MINA ACCOUNTS").click
    expect(page).to have_content("MINA Token Accounts")
  end
end
