require "spec_helper"

RSpec.describe "Meta title", type: :system do
  block_hash = "3NLhBh3d4b91DPoJn5hhwRAWmHSAaG8Qz4W5r9FhJBCXLD3WrAt4"

  pages = [
    {url: "/analytics/staker-leaderboard", title: "Analytics | Staker Leaderboard"},
    {url: "/analytics/snarker-leaderboard", title: "Analytics | Snarker Leaderboard"},
    {url: "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}", title: "Mina Addresses | Search for accounts on Mina Blockchain"},
    {url: "/analytics/commands/user", title: "Analytics | User Commands"},
    {url: "/analytics/blocks", title: "Analytics | Blocks"},
    {url: "/analytics/snarks", title: "Analytics | SNARKs"},
    {url: "/commands/internal", title: "Transactions | Internal Commands"},
    {url: "/blocks", title: "Blocks | Search for blocks on Mina Blockchain"},
    {url: "/blocks/#{block_hash}/", title: "Block Overview | Spotlight"},
    {url: "/blocks/#{block_hash}/commands/user", title: "Block Overview | User Commands"},
    {url: "/blocks/#{block_hash}/commands/internal", title: "Block Overview | Internal Commands"},
    {url: "/blocks/#{block_hash}/snark-jobs", title: "Block Overview | SNARK Jobs"},
    {url: "/broadcast/transaction", title: "Offline Broadcasting | Broadcast Signed Transaction"},
    {url: "/broadcast/delegation", title: "Offline Broadcasting | Broadcast Signed Delegation"},
    {url: "/broadcast/ledger", title: "Offline Broadcasting | Broadcast Signed Transaction From Ledger"},
    {url: "/staking-ledgers?epoch=1", title: "Staking Ledger | Epoch 1"},
    {url: "/snarks", title: "SNARKs | Search For SNARKs"},
    {url: "/commands/#{Constants::FIRST_TXN_HASH}", title: "Transaction Overview | No Memo"},
    {url: "/addresses/accounts/#{Constants::MINA_TOKEN_ADDRESS}/#{Constants::ROMEK_ADDRESS}", title: "Account Overview | #{Constants::ROMEK_USERNAME}"}
  ]

  pages.each do |p|
    it "'#{p[:title]}' exists in <head> for page #{p[:url]}" do
      visit p[:url]
      expect(page).to have_title(p[:title])
    end
  end
end
