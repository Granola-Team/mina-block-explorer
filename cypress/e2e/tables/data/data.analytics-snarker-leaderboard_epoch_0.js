export const url = "/analytics/snarker-leaderboard?epoch=0";
export const table = {
  heading: "Snarker Leaderboard",
  columns: [
    "Username",
    "Public Key",
    "Total Fees",
    "Min Fee",
    "Max Fee",
    "Snarks Sold",
  ],
  sorting_columns: [
    {
      column: "Max Fee",
      type: "numeric",
      sort_options: [null, "MAX_FEE_DESC", "MAX_FEE_ASC"],
    },
    {
      column: "Total Fees",
      type: "numeric",
      sort_options: [null, "TOTAL_FEES_DESC", "TOTAL_FEES_ASC"],
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
