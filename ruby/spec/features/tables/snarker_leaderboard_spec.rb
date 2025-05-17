# spec/features/tables/snarker_leaderboard_spec.rb
require "spec_helper"

RSpec.describe "Snarker Leaderboard table", type: :system do
  let(:url) { "/analytics/snarker-leaderboard?epoch=0" }
  let(:heading) { "Snarker Leaderboard" }
  let(:columns) { ["SNARKer", "Total Fees", "Min Fee", "Max Fee", "Snarks Sold"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has sortable column 'Max Fee'" do
    test_sortable_column(heading, "Max Fee", [nil, "MAX_FEE_DESC", "MAX_FEE_ASC"])
  end

  it "has sortable column 'Total Fees'" do
    test_sortable_column(heading, "Total Fees", [nil, "TOTAL_FEES_DESC", "TOTAL_FEES_ASC"])
  end
end
