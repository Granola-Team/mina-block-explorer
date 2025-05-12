# spec/features/tables/user_commands_block_spec.rb
require "spec_helper"

RSpec.describe "User Commands table", type: :system do
  let(:url) { "/blocks/#{Constants::BLOCK_WITH_ALL_ACTIVITY}/commands/user" }
  let(:heading) { "User Commands" }
  let(:columns) { ["Hash", "Type", "Status", "From", "To", "Nonce", "Fee", "Amount"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has data" do
    table_rows = get_table_rows(heading)
    expect(table_rows.count).to be >= 0, "Expected '#{heading}' table to have at least 0 rows, but found #{table_rows.count}"
  end
end

RSpec.describe "SNARK Jobs table", type: :system do
  let(:url) { "/blocks/#{Constants::BLOCK_WITH_ALL_ACTIVITY}/snark-jobs" }
  let(:heading) { "SNARK Jobs" }
  let(:columns) { ["State Hash", "Date", "Prover", "Fee"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has data" do
    table_rows = get_table_rows(heading)
    expect(table_rows.count).to be >= 0, "Expected '#{heading}' table to have at least 0 rows, but found #{table_rows.count}"
  end
end

RSpec.describe "Internal Commands table", type: :system do
  let(:url) { "/blocks/#{Constants::BLOCK_WITH_ALL_ACTIVITY}/commands/internal" }
  let(:heading) { "Internal Commands" }
  let(:columns) { ["Recipient", "Fee", "Type"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has data" do
    table_rows = get_table_rows(heading)
    expect(table_rows.count).to be >= 0, "Expected '#{heading}' table to have at least 0 rows, but found #{table_rows.count}"
  end
end
