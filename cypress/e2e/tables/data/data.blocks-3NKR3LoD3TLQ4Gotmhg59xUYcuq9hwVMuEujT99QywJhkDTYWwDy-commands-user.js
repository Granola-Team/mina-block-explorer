export const tag = "@tier2";
export const url =
  "/blocks/3NKR3LoD3TLQ4Gotmhg59xUYcuq9hwVMuEujT99QywJhkDTYWwDy/commands/user";
export const exclusive = true;
export const table = {
  heading: "User Commands",
  columns: ["Hash", "Type", "Status", "From", "To", "Nonce", "Fee", "Amount"],
  filter_tests: [],
};
export const tests = [
  () => {
    cy.aliasTableRows("User Commands", "table-rows");
    cy.get("@table-rows").should("have.lengthOf", 6);
    cy.get("@table-rows").eq(0).should("contain.text", "Failed");
    cy.get("@table-rows").eq(5).should("contain.text", "Applied");
  },
];
export default {
  tag,
  url,
  exclusive,
  table,
  tests,
};
