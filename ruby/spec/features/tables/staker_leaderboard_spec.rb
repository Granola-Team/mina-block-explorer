# spec/features/tables/snarker_leaderboard_spec.rb
require "spec_helper"

RSpec.describe "Snarker Leaderboard table", type: :system do
  let(:url) { "/analytics/staker-leaderboard?epoch=0" }
  let(:heading) { "Staker Leaderboard" }
  let(:columns) {
    [
      "Delegate",
      "Canonical Blocks Produced",
      "Stake Percentage",
      "Slots Produced",
      "Orphan Rate",
      "Supercharged Blocks Produced"
    ].map(&:upcase)
  }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has sortable column 'Canonical Blocks Produced'" do
    test_sortable_column(heading, "Canonical Blocks Produced", [nil, "NUM_CANONICAL_BLOCKS_PRODUCED_ASC", "NUM_CANONICAL_BLOCKS_PRODUCED_DESC"])
  end
end
