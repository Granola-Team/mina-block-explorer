import { ZK_APP_TXN_HASH } from "../../constants";
export const tag = "@tier2";
export const url = `/commands/${ZK_APP_TXN_HASH}`;
export const table = {
  heading: "Accounts Updated",
  columns: ["Account", "Balance Change", "Increment Nonce", "Token ID"],
  filter_tests: [],
};
export const tests = [];
export default {
  tag,
  url,
  table,
  tests,
};
