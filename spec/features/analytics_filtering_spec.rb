require "spec_helper"

RSpec.describe "Block height filters", type: :system do
  pages = [
    "/analytics/blocks",
    "/analytics/commands/user",
    "/analytics/snarks"
  ].freeze

  pages.each do |url|
    it "work on #{url}" do
      visit url
      start_block_height = get_by_sel("start-block-height-input")
      end_block_height = get_by_sel("end-block-height-input")
      submit_button = find("button", text: "Apply")

      start_block_height.set("")
      end_block_height.set("")

      submit_button.click
      expect(page).to have_css("#input-validation", text: "Missing start block height")

      start_block_height.set("")
      start_block_height.set("9000")
      expect(page).not_to have_css("#input-validation")
      submit_button.click
      expect(page).to have_css("#input-validation", text: "Missing end block height")

      end_block_height.set("")
      end_block_height.set("9000")
      expect(page).not_to have_css("#input-validation")
      submit_button.click
      expect(page).to have_css("#input-validation", text: "End block must be larger than start block")

      end_block_height.set("")
      end_block_height.set("9001")
      expect(page).not_to have_css("#input-validation")
      submit_button.click
      expect(page).not_to have_css("#input-validation")

      start_block_height.set("")
      start_block_height.set("6000")
      expect(page).not_to have_css("#input-validation")
      submit_button.click
      expect(page).to have_css("#input-validation", text: "Block range must not exceed 2000")
    end
  end
end

RSpec.describe "Block height filter URL params", type: :system do
  cases = [
    {url: "/analytics/blocks?q-blockheight-gte=7000", expected_gte_input: 7000, expected_lte_input: nil},
    {url: "/analytics/blocks?q-blockheight-lte=9050", expected_gte_input: nil, expected_lte_input: 9050},
    {url: "/analytics/blocks?q-blockheight-gte=8050&q-blockheight-lte=9050", expected_gte_input: 8050, expected_lte_input: 9050}
  ].freeze

  cases.each do |test_case|
    it "work for #{test_case[:url]}" do
      visit test_case[:url]
      start_block_height = find("label", text: "Start Block Height").all(:xpath, "./following-sibling::*").first
      end_block_height = find("label", text: "End Block Height").all(:xpath, "./following-sibling::*").first

      if test_case[:expected_gte_input].nil?
        expect(start_block_height.value).to be_empty
      else
        expect(start_block_height.value).to eq(test_case[:expected_gte_input].to_s)
      end

      if test_case[:expected_lte_input].nil?
        expect(end_block_height.value).to be_empty
      else
        expect(end_block_height.value).to eq(test_case[:expected_lte_input].to_s)
      end
    end
  end
end
