export const url = "/commands/pending";
export const table = {
  heading: "Pending Commands",
  columns: ["Txn Hash", "Type", "From", "To", "Nonce", "Fee", "Amount"],
  filter_tests: [],
};
export const tests = [];
export default {
  url,
  table,
  tests,
};
