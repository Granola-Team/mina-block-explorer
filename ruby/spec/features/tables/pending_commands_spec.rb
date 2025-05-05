# spec/features/tables/pending_commands_spec.rb
require "spec_helper"

RSpec.describe "Pending Commands table", type: :system do
  let(:url) { "/commands/pending" }
  let(:heading) { "Pending Commands" }
  let(:columns) { ["Txn Hash", "Type", "From", "To", "Nonce", "Fee", "Amount"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end
end
