require "spec_helper"

RSpec.describe "Empty table", type: :system do
  pages = [
    "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}?q-account=B62fake",
    "/blocks?q-state-hash=3Nfake",
    "/commands/user?q-txn-hash=Cpkfake",
    "/commands/internal?q-recipient=B62qfake",
    "/staking-ledgers?q-account=B62qfake",
    "/snarks?q-state-hash=3Nfake",
    "/analytics/staker-leaderboard?epoch=100000",
    "/analytics/snarker-leaderboard?epoch=100000"
  ]

  pages.each do |url|
    it "on #{url} shows as having zero records" do
      visit url
      expect(page).to have_content("No data for this view", wait: 30)
    end
  end
end
