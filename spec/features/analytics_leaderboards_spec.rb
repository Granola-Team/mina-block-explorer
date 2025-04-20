require "spec_helper"

RSpec.describe "Analytics Leaderboard", type: :system do
  leaderboards = [
    {name: "Staker Leaderboard", selector: "staker-leaderboard-tab"},
    {name: "Snarker Leaderboard", selector: "staker-leaderboard-tab"}
  ]

  leaderboards.each do |leaderboard|
    it "#{leaderboard[:name]} defaults to latest epoch" do
      visit "/analytics/"
      get_by_sel(leaderboard[:selector]).click
      expect(page).to have_current_path(/epoch=0/)
      epoch_input = get_by_sel("epoch-input")
      expect(epoch_input.value).to eq("0")
    end
  end
end
