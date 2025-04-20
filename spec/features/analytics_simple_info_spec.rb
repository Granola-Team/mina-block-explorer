require "spec_helper"

PAGES = [
  "/blocks/#{APPLIED_TXN_BLOCK_STATE_HASH}/analytics",
  "/analytics/snarks?q-blockheight-gte=359606&q-blockheight-lte=359706",
  "/analytics/commands/user?q-blockheight-gte=0&q-blockheight-lte=10000"
].freeze

RSpec.describe "Analytics Simple Info stats", type: :system do
  PAGES.each do |page|
    it "renders on #{page}" do
      visit page
      get_all_by_sel("analytics-simple-info").each do |element|
        is_numeric?(element)
      end
    end
  end
end
