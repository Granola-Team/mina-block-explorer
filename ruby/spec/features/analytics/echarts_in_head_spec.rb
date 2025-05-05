require "spec_helper"

RSpec.describe "<head>", type: :system do
  sample_non_charting_pages = ["/", "/blocks"]
  charting_pages = ["/analytics/blocks"]

  sample_non_charting_pages.each do |url|
    it "does not contain charting libraries on non-charting page #{url}" do
      visit url
      expect(page).not_to have_css('head script[src*="echarts"]', wait: 2, visible: false)
    end
  end

  charting_pages.each do |url|
    it "contains charting libraries on #{url}" do
      visit url
      expect(page).to have_css('head script[src*="echarts"]', wait: 2, visible: false)
    end
  end
end
