export const url = "/analytics/staker-leaderboard?epoch=0";
export const table = {
  heading: "Staker Leaderboard",
  columns: [
    "Username",
    "Public Key",
    "Canonical Blocks Produced",
    "Supercharged Blocks Produced",
    "Slots Produced",
    "Orphan Rate",
  ],
  filter_tests: [],
};
export const tests = [];
export default {
  url,
  table,
  tests,
};
