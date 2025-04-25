require "spec_helper"

RSpec.describe "Tokens page navigation", type: :system do
  # Reusable method to verify tokens page navigation
  def verify_tokens_page
    expect(page.current_url).to include("/tokens?q-id=#{Constants::NFT_TOKEN_ID}"), "Expected URL to include '/tokens?q-id=#{Constants::NFT_TOKEN_ID}', but was #{page.current_url}"

    # Verify the "ID" column header contains an input with the correct value
    id_header = find("th", text: "ID")
    input = id_header.find("input")
    expect(input.value).to eq(Constants::NFT_TOKEN_ID), "Expected 'ID' input to have value '#{Constants::NFT_TOKEN_ID}', but was '#{input.value}'"
  end

  it "is navigated to from command spotlight page" do
    visit "/commands/#{Constants::ZK_APP_TXN_HASH}"

    # Wait for the "Accounts Updated" table to load
    wait_until_table_loaded("Accounts Updated")

    # Click the link in the "Token ID" column of the 6th row.
    click_link_in_table_column("Accounts Updated", "Token ID".upcase, 6)

    # Verify navigation to the tokens page
    verify_tokens_page
  end

  # TODO: uncomment when https://github.com/Granola-Team/mina-indexer/issues/1860 closed and
  # indexer version is updated locally in project
  xit "is navigated to from the account token holding page" do
    visit "/addresses/accounts/#{Constants::TOKEN_ACTIVITY_ONLY_ADDRESS}/tokens"

    # Wait for the "Tokens" table to load
    wait_until_table_loaded("Tokens")

    # Click the link in the "Token ID" column of the 1st row.
    click_link_in_table_column("Tokens", "Token ID".upcase, 1)

    # Verify navigation to the tokens page
    verify_tokens_page
  end
end
