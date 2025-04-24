require "spec_helper"

RSpec.describe "Snarks page navigation", type: :system do
  pages = [
    {
      origin: "/addresses/accounts/#{Constants::GENESIS_ACCOUNT_PK}/snark-jobs",
      dest: "snarks",
      href: "/snarks?q-prover=#{Constants::GENESIS_ACCOUNT_PK}"
    }
  ]

  pages.each do |test_case|
    it "is navigated to from #{test_case[:dest]}" do
      visit test_case[:origin]

      wait_until_table_loaded("SNARK Jobs")

      # Click the link with text "See all snark jobs"
      find("a", text: "See all snark jobs".upcase).click

      # Verify the URL contains the expected href
      expect(page.current_url).to include(test_case[:href]), "Expected URL to include '#{test_case[:href]}', but was #{page.current_url}"

      # Verify the "Prover" column in the "SNARKs" table contains GENESIS_ACCOUNT_PK
      wait_until_table_loaded("SNARKs")
      prover_cells = all(table_column_selector("SNARKs", "Prover".upcase))
      prover_cells.each do |cell|
        # Join split text if necessary (as per your previous issue)
        cleaned_text = cell.text.gsub(/[\n+-]/, "")
        expect(cleaned_text).to eq(Constants::GENESIS_ACCOUNT_PK), "Expected 'Prover' column to contain '#{Constants::GENESIS_ACCOUNT_PK}', but found '#{cleaned_text}'"
      end
    end
  end
end
