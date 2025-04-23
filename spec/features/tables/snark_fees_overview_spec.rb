# spec/features/tables/snark_fees_overview_spec.rb
require "spec_helper"

RSpec.describe "SNARK Fees Overview table", type: :system do
  let(:url) { "/analytics/snarks" }
  let(:heading) { "SNARK Fees Overview" }
  let(:columns) { ["Metric", "All SNARKs", "SNARKs with non-zero fees"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end
end
