// Test suite data for: /tokens

module.exports = {
  tag: "@tier2",
  url: "/tokens",
  table: {
    heading: "Tokens",
    columns: [
      "Name",
      "ID",
      "Supply",
      "Owner",
      "Holders",
      "Transactions",
      "Locked",
    ],
    // sorting_columns: [
    //   {
    //     column: "Balance",
    //     type: "numeric",
    //     sort_options: ["BALANCE_DESC", "BALANCE_ASC"],
    //   },
    // ],
    filter_tests: [
      //   {
      //     column: "Balance",
      //     input: 5000,
      //     assertion: function () {
      //       cy.aliasTableRows("Accounts", "table-rows");
      //       cy.get("@table-rows").should("have.lengthOf", 25);
      //       cy.assertForEachColumnValue("Accounts", "Balance", (text) => {
      //         let balance = parseFormattedNumber(text);
      //         expect(balance).to.be.lte(5000);
      //       });
      //     },
      //   },
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
