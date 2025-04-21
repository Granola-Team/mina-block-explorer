require "spec_helper"

RSpec.describe "Global search", type: :system do
  it "has visible placeholder text" do
    page.driver.resize(414, 896) # iphone-xr
    visit "/"
    expect(page).to have_css('input[placeholder="Paste -> Enter -> Explore!"]', visible: true)
  end

  xyz_tokens = [MINU_TOKEN_ADDRESS, TITS_TOKEN_ID, NFT_TOKEN_ID]
  items = xyz_tokens.map do |token|
    {input: token, expected_url: "/tokens?q-id=#{token}"}
  end
  items.push(
    {input: "359617", expected_url: "/blocks?q-height=359617"},
    {input: "      #{EPOCH_ZERO_STAKING_LEDGER_HASH}     ", expected_url: "/staking-ledgers?epoch=0"},
    {input: EPOCH_ZERO_STAKING_LEDGER_HASH, expected_url: "/staking-ledgers?epoch=0"},
    {input: GENESIS_ACCOUNT_PK, expected_url: "/addresses/accounts/#{GENESIS_ACCOUNT_PK}"},
    {input: GENESIS_BLOCK_BLOCK_HASH, expected_url: "/blocks/#{GENESIS_BLOCK_BLOCK_HASH}"},
    {input: FIRST_TXN_HASH, expected_url: "/commands/#{FIRST_TXN_HASH}"},
    {input: "1", expected_url: "/staking-ledgers?epoch=1"}
  )

  items.each do |item|
    it "works for input #{item[:input]}" do
      visit "/"
      input_field = find("input#searchbar")
      input_field.set(item[:input])
      input_field.send_keys(:enter)
      expect(page).to have_current_path(/#{Regexp.escape(item[:expected_url])}/, wait: 5)
      expect(input_field.value).not_to eq(item[:input])
    end
  end
end
