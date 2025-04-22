require "spec_helper"

RSpec.describe "Mobile menu", type: :system do
  before do
    # Set viewport to iPhone XR dimensions (414x896, as defined in Constants::DEVICES)
    page.driver.resize(414, 896)

    visit "/"

    # Verify the nav is not visible initially
    expect(page).not_to have_selector("nav", visible: true), "Expected nav to not be visible initially"

    # Toggle the mobile menu
    find("[data-test=\"mobile-menu-toggle\"]", wait: 1).click

    # Verify the nav is now visible
    expect(page).to have_selector("nav", visible: true), "Expected nav to be visible after toggling mobile menu"
  end

  Constants::TOP_LEVEL_PAGES.each do |url|
    it "provides navigation to #{url}" do
      # Click the first nav link matching the URL
      all("nav a[href^='#{url}']").first.click

      # Verify the URL contains the expected path
      expect(page.current_path).to eq(url), "Expected URL to be '#{url}', but was #{page.current_path}"
    end
  end

  it "has all menu items visible" do
    # Find all menu item spans and verify each is visible
    menu_items = find_all("a.nav-link span")
    menu_items.each do |item|
      expect(item).to be_visible, "Expected menu item '#{item.text}' to be visible"
    end
  end
end
