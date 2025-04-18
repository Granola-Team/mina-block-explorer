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
  sorting_columns: [
    {
      column: "Canonical Blocks Produced",
      type: "numeric",
      sort_options: [
        "NUM_CANONICAL_BLOCKS_PRODUCED_DESC",
        "NUM_CANONICAL_BLOCKS_PRODUCED_ASC",
      ],
    },
  ],
  filter_tests: [],
};
export const tests = [];
export default {
  url,
  table,
  tests,
};
