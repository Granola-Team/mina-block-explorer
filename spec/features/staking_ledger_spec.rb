require "spec_helper"

RSpec.describe "Staking ledger", type: :system do
  def extract_epoch_progress(input)
    # Match the format "X% complete (Y/Z slots filled)"
    regex = /(\d{1,3}(?:,\d{3})*|\d+)(?:\.(\d+))?% complete \((\d{1,3}(?:,\d{3})*)\/(\d{1,3}(?:,\d{3})*) slots filled\)/
    match = input.match(regex)
    return unless match

    {
      percent: "#{match[1]}#{match[2] ? ".#{match[2]}" : ""}", # Append decimal part if it exists
      slot: match[3].delete(","),
      total_slots: match[4].delete(",")
    }
  end

  before(:each) do
    visit "/staking-ledgers"

    wait_until_table_loaded("Staking Ledger - Epoch 0")

    # Wait for the /summary API request by ensuring a summary element is present
    expect(page).to have_selector(".ledger-hash", wait: 5)
  end

  it "displays a ledger hash" do
    expect(page).to have_selector(".ledger-hash"), "Expected '.ledger-hash' element to be present"
  end

  it "shows slot progress message" do
    slot_info = find(".staking-ledger-percent-complete")
    epoch_progress_text = slot_info.text

    info = extract_epoch_progress(epoch_progress_text)
    expect(info).not_to be_nil, "Failed to parse epoch progress from '#{epoch_progress_text}'"

    calculated_percent = ((info[:slot].to_f / info[:total_slots].to_f) * 100).round(0)
    percent = info[:percent].to_f
    expect(percent).to be_within(1).of(calculated_percent), "Expected percent '#{percent}' to be within 1 of calculated percent '#{calculated_percent}'"
  end

  it "defaults to current epoch" do
    expect(page).to have_selector("section", text: "Staking Ledger"), "Expected 'Staking Ledger' section to be present"
  end

  it "disables 'Previous' button appropriately" do
    expect(page).to have_selector("button.hover\\:cursor-not-allowed", text: "Previous"), "Expected 'Previous' button to be disabled"
    expect(page).not_to have_selector("button.hover\\:cursor-not-allowed", text: "Next"), "Expected 'Next' button to not be disabled"
  end

  it "contains buttons for epoch navigation" do
    expect(page).to have_selector("section", text: "Staking Ledger - Epoch 0"), "Expected 'Staking Ledger - Epoch 0' to be present"

    find("section button", text: "Next", match: :first).click
    sleep 0.5 # Equivalent to cy.wait(500)

    expect(page).to have_selector("section", text: "Staking Ledger - Epoch 1"), "Expected 'Staking Ledger - Epoch 1' to be present after clicking 'Next'"

    # TODO: Enable when more epochs are available
    # find("section button", text: "Previous").click
    # sleep 0.5
    # expect(page).to have_selector("section", text: "Staking Ledger - Epoch 0"), "Expected 'Staking Ledger - Epoch 0' to be present after clicking 'Previous'"
  end

  xit "disables 'Next' button appropriately" do
    sleep 0.5
    find("section button", text: "Next").click

    expect(page).not_to have_selector("button.hover\\:cursor-not-allowed", text: "Previous"), "Expected 'Previous' button to not be disabled"
    expect(page).to have_selector("button.hover\\:cursor-not-allowed", text: "Next"), "Expected 'Next' button to be disabled"
  end

  it "displays 0% complete for the next epoch" do
    visit "/staking-ledgers?epoch=0"
    expect(page).to have_content("40% complete"), "Expected '40% complete' to be present for epoch 0"

    find("button", text: "Next").click
    expect(page).to have_content("0% complete"), "Expected '0% complete' to be present for the next epoch"
  end
end
