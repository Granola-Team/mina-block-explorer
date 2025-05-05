require "spec_helper"

RSpec.describe "Chart", type: :system do
  analytics_pages = [
    {
      url: "/analytics/commands/user",
      chart_ids: [
        "#user-commands-volume",
        "#user-commands-top-recipients",
        "#user-commands-top-transfers",
        "#fee-spread",
        "#transfer-count"
      ]
    },
    {
      url: "/analytics/blocks",
      chart_ids: [
        "#rewards",
        "#blocks",
        "#tree",
        "#top-block-producers",
        "#top-block-earners"
      ]
    },
    {
      url: "/analytics/snarks",
      chart_ids: [
        "#avg-snark-fee",
        "#fees-per-block",
        "#fee-distribution",
        "#snark-jobs-count",
        "#top-snark-provers",
        "#top-snark-workers"
      ]
    }
  ]

  analytics_pages.each do |analytics_page|
    it "renders on page #{analytics_page[:url]}" do
      visit analytics_page[:url]
      analytics_page[:chart_ids].each do |id|
        expect(page).to have_css("#{id} canvas", visible: :visible, wait: 10)
      end
    end
  end
end
