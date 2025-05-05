require "spec_helper"

RSpec.describe "Blockchain overview", type: :system do
  it "displays valid metrics" do
    visit "/blocks"

    # Wait for the "Blocks" table to load (ensures page content is ready)
    wait_until_table_loaded("Blocks")

    # Wait for the /summary API request to complete by checking for a summary element
    # Assuming #epoch is populated after the /summary request (adjust selector if needed)
    expect(page).to have_selector("#epoch", wait: 5)

    numeric_summary_items = [
      {id: "#epoch", label: "Epoch"},
      {id: "#uniqueBlockProducers", label: "Unique Producers of last 10000 blocks"},
      {id: "#globalSlot", label: "Global Slot"},
      {id: "#blockchainLength", label: "Blockchain Length"},
      {id: "#totalMina", label: "Total MINA"},
      {id: "#circulatingSupply", label: "Circulating Supply"},
      {id: "#totalNumBlocks", label: "Total Blocks"},
      {id: "#totalUserCommands", label: "Total User Commands"},
      {id: "#totalInternalCommands", label: "Total Internal Commands"},
      {id: "#totalSnarks", label: "Total SNARKs"}
    ]

    string_summary_items = [
      {id: "#chainId", label: "Chain ID"},
      {id: "#genesisStateHash", label: "Genesis State Hash"}
    ]

    numeric_summary_items.each do |item|
      element = find(item[:id])
      is_numeric?(element) # Uses CapybaraHelpers method to verify the element's text is numeric
      label = element.sibling("label")
      expect(label.text).to eq(item[:label]), "Expected label for '#{item[:id]}' to be '#{item[:label]}', but was '#{label.text}'"
    end

    string_summary_items.each do |item|
      element = find(item[:id])
      expect(element.text).not_to be_empty, "Expected '#{item[:id]}' to not be empty, but it was"
      label = element.sibling("label")
      expect(label.text).to eq(item[:label]), "Expected label for '#{item[:id]}' to be '#{item[:label]}', but was '#{label.text}'"
    end
  end
end
