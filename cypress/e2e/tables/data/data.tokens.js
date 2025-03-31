// Test suite data for: /tokens

module.exports = {
  tag: "@tier2",
  url: "/tokens",
  table: {
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
        input: "MINA",
        assertion: function () {
          cy.aliasTableRows("Tokens", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 1);
          cy.assertForEachColumnValue("Tokens", "Symbol", (text) => {
            expect(text).to.be.eq("MINA");
          });
        },
      },
    ],
  },
  tests: [
    () => {
      // [30].forEach((l) => {
      //   cy.assertRowLimitWorks("Tokens", l);
      // });
    },
    // () => {
    //   cy.intercept("GET", "/rest").as("rest");
    //   cy.visit("/tokens");
    //   cy.wait("@graphql").then(() => {
    //     cy.wait(1000);
    //     cy.assertLoadNextWorks("Tokens", "Balance");
    //   });
    // },
  ],
};
