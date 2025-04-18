import { ZK_APP_TXN_HASH } from "../../constants";
export const url = `/commands/${ZK_APP_TXN_HASH}`;
export const table = {
  heading: "Accounts Updated",
  columns: ["Account", "Balance Change", "Increment Nonce", "Token ID"],
  filter_tests: [],
};
export const tests = [];
export default {
  url,
  table,
  tests,
};
