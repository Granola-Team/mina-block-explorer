require "spec_helper"

RSpec.describe "Desktop footer", type: :system do
  Constants::DEVICES.each do |device_name, (width, height)|
    context "on device #{device_name}" do
      before do
        page.driver.resize(width, height)
      end

      Constants::TOP_LEVEL_PAGES.each do |p|
        it "exists on #{p} page" do
          visit p
          page.has_selector?("footer")
        end
      end
    end
  end
end

RSpec.describe "Footer links", type: :system do
  it "links to deployed version on github.com" do
    # Visit the homepage
    visit "/"

    # Wait for the mina-block-explorer commit link to appear and find it (up to 10 seconds)
    explorer_link = find(
      "footer a[href^='https://github.com/Granola-Team/mina-block-explorer/commit/']",
      wait: 10
    )

    # Verify the href matches the expected pattern
    explorer_href = explorer_link[:href]
    expect(explorer_href).to match(/https:\/\/github\.com\/Granola-Team\/mina-block-explorer\/commit\/[a-f0-9]{8}$/), "Expected explorer commit link to match pattern, but was '#{explorer_href}'"

    # Wait for the mina-indexer commit link to appear (indicating /summary request completion)
    indexer_link = find(
      "footer a[href^='https://github.com/Granola-Team/mina-indexer/commit/']",
      wait: 10
    )

    # Verify the href matches the expected pattern
    indexer_href = indexer_link[:href]
    expect(indexer_href).to match(/https:\/\/github\.com\/Granola-Team\/mina-indexer\/commit\/[a-f0-9]{7,40}$/), "Expected indexer commit link to match pattern, but was '#{indexer_href}'"
  end
end

RSpec.describe "Footer content", type: :system do
  let(:footer_links) do
    [
      {text: "Granola", selector: 'a[href="https://granola.team"]'},
      {text: "DOCS", selector: 'a[href="https://docs.minasearch.com"]'},
      {text: "API", selector: 'a[href="https://docs.minasearch.com/apis"]'},
      {text: "DISCORD", selector: 'a[href="https://discord.gg/Zvu6XHNCxj"]'}
    ]
  end

  it "is present and visible on mobile" do
    # Set viewport to iPhone XR dimensions (414x896, as defined in Constants::DEVICES)
    page.driver.resize(414, 896)

    # Visit the homepage
    visit "/"

    # Verify each footer link is present and visible
    footer_links.each do |link|
      find("footer #{link[:selector]}", text: link[:text], wait: 1, visible: true)
    end
  end

  it "is present and visible on desktop" do
    # Use a default desktop viewport (e.g., MacBook-11 dimensions: 1366x768 from Constants::DEVICES)
    page.driver.resize(1366, 768)

    # Visit the homepage
    visit "/"

    # Verify each footer link is present and visible
    footer_links.each do |link|
      find("footer #{link[:selector]}", text: link[:text], wait: 1, visible: true)
    end
  end
end
