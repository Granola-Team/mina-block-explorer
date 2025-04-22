require "spec_helper"

RSpec.describe "Not Found Page", type: :system do
  pages = [
    {
      url: "/commands/GggGXNjmeiA59Kn1qiyG3NZ1oT1sBNBg8iwvLzJuyT7GH9dVmGggg",
      message: "Transaction Not Found :("
    }
  ]

  pages.each do |test_case|
    it "displays on #{test_case[:url]}" do
      visit test_case[:url]

      # Verify the error message is displayed on the page
      expect(page).to have_content(test_case[:message]), "Expected page to contain '#{test_case[:message]}'"
    end
  end
end
