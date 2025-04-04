import { BLOCK_WITH_ALL_ACTIVITY } from "../../constants";

export const tag = "@tier2";
export const url = `/blocks/${BLOCK_WITH_ALL_ACTIVITY}/commands/user`;
export const exclusive = true;
export const table = {
  heading: "User Commands",
  columns: ["Hash", "Type", "Status", "From", "To", "Nonce", "Fee", "Amount"],
  filter_tests: [],
};
export const tests = [
  () => {
    cy.aliasTableRows("User Commands", "table-rows");
    cy.get("@table-rows").should("have.length.gte", 0);
  },
];
export default {
  tag,
  url,
  exclusive,
  table,
  tests,
};
