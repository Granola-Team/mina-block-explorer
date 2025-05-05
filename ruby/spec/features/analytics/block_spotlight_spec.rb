require "spec_helper"

RSpec.describe "Block analytic tab", type: :system do
  it "contains the correct elements" do
    # Visit the analytics tab for the genesis block
    visit "/blocks/#{Constants::GENESIS_BLOCK_BLOCK_HASH}/analytics"

    # Verify there are 4 elements with class "analytics-sm"
    small_analytics = find_all(".analytics-sm")
    expect(small_analytics.count).to eq(4), "Expected 4 elements with class 'analytics-sm', but found #{small_analytics.count}"

    # Verify there are 2 elements with class "analytics-lg"
    large_analytics = find_all(".analytics-lg")
    expect(large_analytics.count).to eq(2), "Expected 2 elements with class 'analytics-lg', but found #{large_analytics.count}"
  end
end
