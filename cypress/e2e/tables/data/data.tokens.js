import { MINU_TOKEN_ADDRESS } from "../../constants";
export const url = "/tokens";
export const table = {
  heading: "Tokens",
  columns: [
    "Symbol",
    "Supply",
    "ID",
    "Owner",
    "Holders",
    "Transactions",
    "% Unlocked",
  ],
  // sorting_columns: [
  //   {
  //     column: "Balance",
  //     type: "numeric",
  //     sort_options: ["BALANCE_DESC", "BALANCE_ASC"],
  //   },
  // ],
  filter_tests: [
    {
      column: "Symbol",
      input: "MINU",
      assertion: function () {
        cy.aliasTableRows("Tokens", "table-rows");
        cy.get("@table-rows").should("have.lengthOf", 1);
        cy.assertForEachColumnValue("Tokens", "Symbol", (text) => {
          expect(text).to.be.eq("MINU");
        });
      },
    },
    {
      column: "ID",
      input: MINU_TOKEN_ADDRESS,
      assertion: function () {
        cy.aliasTableRows("Tokens", "table-rows");
        cy.get("@table-rows").should("have.lengthOf", 1);
        cy.assertForEachColumnValue("Tokens", "ID", (text) => {
          expect(text).to.be.eq(MINU_TOKEN_ADDRESS);
        });
      },
    },
  ],
};
export const tests = [];
export default {
  url,
  table,
  tests,
};
