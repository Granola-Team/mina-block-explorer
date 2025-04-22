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
