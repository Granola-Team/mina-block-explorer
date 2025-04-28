# spec/features/tables/staking_ledger_epoch_1_spec.rb
require "spec_helper"

RSpec.describe "Staking Ledger - Epoch 1 table", type: :system do
  let(:url) { "/staking-ledgers?epoch=1" }
  let(:heading) { "Staking Ledger - Epoch 1" }
  let(:columns) { ["Key", "Username", "Balance", "Stake", "Total Stake %", "Block Win %", "Delegate", "Delegators"].map(&:upcase) }

  before do
    visit url
    wait_until_table_loaded(heading)
  end

  it "has ordered columns" do
    test_ordered_columns(heading, columns)
  end

  it "has sortable column 'Total Stake %'" do
    test_sortable_column(heading, "Total Stake %", ["STAKE_DESC", "STAKE_ASC"])
  end

  it "has working filter for column 'Key' with input 'B62qq3tqfdj19hqaVCozJFM2q9gT2WezQMaJMKD6wxyvK3fMpHiP9va'" do
    test_filter(
      heading,
      "Key",
      "B62qq3tqfdj19hqaVCozJFM2q9gT2WezQMaJMKD6wxyvK3fMpHiP9va",
      nil,
      lambda do
        metadata = get_table_metadata("Staking Ledger - Epoch 1")
        expect(metadata.length).to eq(2), "Expected 'Staking Ledger - Epoch 1' table metadata to have 2 datum, but found #{metadata.length}"
        table_rows = get_table_rows("Staking Ledger - Epoch 1")
        expect(table_rows.count).to eq(1), "Expected 'Staking Ledger - Epoch 1' table to have 1 row, but found #{table_rows.count}"
        key_cells = all(table_column_selector("Staking Ledger - Epoch 1", "Key".upcase))
        key_cells.each do |cell|
          cleaned_text = cell.text.gsub(/[\n+-]/, "")
          expect(cleaned_text).to eq(Constants::SNZPOOL_ADDRESS), "Expected 'Key' to be '#{Constants::SNZPOOL_ADDRESS}', but was '#{cleaned_text}'"
        end
        username_cells = all(table_column_selector("Staking Ledger - Epoch 1", "Username".upcase))
        username_cells.each do |cell|
          expect(cell.text).to eq(Constants::SNZ_USERNAME), "Expected 'Username' to be '#{Constants::SNZ_USERNAME}', but was '#{cell.text}'"
        end
      end
    )
  end

  # TODO: Fix broken selector
  xit "has working filter for column 'Stake' with input '7,399,987.246422696'" do
    test_filter(
      heading,
      "Stake",
      "7,399,987.246422696",
      nil,
      lambda do
        metadata = get_table_metadata("Staking Ledger - Epoch 1")
        expect(metadata.length).to eq(2), "Expected 'Staking Ledger - Epoch 1' table metadata to have 2 datum, but found #{metadata.length}"
        stake_cells = all(table_column_selector("Staking Ledger - Epoch 1", "Stake".upcase), match: :first)
        stake_cells.each do |cell|
          stake = cell.text.delete(",").to_f
          expect(stake).to be <= 7_399_987.246422696, "Expected stake '#{stake}' to be <= 7,399,987.246422696"
        end
      end
    )
  end

  it "has working filter for column 'Delegate' with input 'B62qjCuPisQjLW7YkB22BR9KieSmUZTyApftqxsAuB3U21r3vj1YnaG'" do
    test_filter(
      heading,
      "Delegate",
      "B62qjCuPisQjLW7YkB22BR9KieSmUZTyApftqxsAuB3U21r3vj1YnaG",
      nil,
      lambda do
        metadata = get_table_metadata("Staking Ledger - Epoch 1")
        expect(metadata.length).to eq(2), "Expected 'Staking Ledger - Epoch 1' table metadata to have 2 datum, but found #{metadata.length}"
      end
    )
  end

  it "has 3 metadata points" do
    visit "/staking-ledgers?epoch=1"
    sleep 1 # Equivalent to cy.wait(1000)
    metadata = get_table_metadata("Staking Ledger - Epoch 1")
    expect(metadata.length).to eq(3), "Expected 'Staking Ledger - Epoch 1' table metadata to have 3 datum, but found #{metadata.length}"
  end

  it "has standard row limits" do
    select_input = get_by_sel("row-limit")
    [25, 50, 100, 250, 500, 1000].each { |limit|
      select_option(select_input, limit.to_s)
      wait_until_table_loaded("Staking Ledger - Epoch 1")
      table_rows = get_table_rows("Staking Ledger - Epoch 1")
      expect(table_rows.count).to be == limit, "Expected 'Staking Ledger - Epoch 1' table to have row count of #{limit}, but found #{table_rows.count}"
    }
  end
end
