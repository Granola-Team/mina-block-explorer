require "spec_helper"

RSpec.describe "Snark fees", type: :system do
  it "are rendered" do
    # Visit the SNARK fees analytics page with query parameters
    visit "/analytics/snarks?q-blockheight-gte=8000&q-blockheight-lte=10000"

    # Wait for the "SNARK Fees Overview" table to load
    wait_until_table_loaded("SNARK Fees Overview")

    # Get all rows in the "SNARK Fees Overview" table (excluding header row)
    table_rows = find_all("table[data-test='#{to_kebab_case("SNARK Fees Overview")}-table'] tr:not(:has(th))")

    # Verify the table has 8 rows
    expect(table_rows.count).to eq(8), "Expected 8 rows in 'SNARK Fees Overview' table, but found #{table_rows.count}"

    # Define the metrics to check in each row
    metrics = ["Count", "Sum", "Mean", "Median", "Min", "Max", "25%", "75%"]

    # Verify each row contains the corresponding metric
    metrics.each_with_index do |metric, i|
      row_text = table_rows[i].text
      expect(row_text).to include(metric), "Expected row #{i + 1} to contain '#{metric}', but found '#{row_text}'"
    end
  end
end
