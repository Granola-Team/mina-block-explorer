// Test suite data for: /addresses/accounts
const { parseFormattedNumber } = require("../../helpers");
const { ROMEK_ADDRESS, ROMEK_USERNAME } = require("../../constants");

module.exports = {
  tag: "@tier2",
  url: "/addresses/accounts",
  table: {
    heading: "Mina Accounts",
    columns: [
      "Public Key",
      "Username",
      "Balance",
      "Nonce",
      "Delegate",
      "Time Locked",
    ],
    sorting_columns: [
      {
        column: "Balance",
        type: "numeric",
        sort_options: ["BALANCE_DESC", "BALANCE_ASC"],
      },
    ],
    filter_tests: [
      {
        column: "Balance",
        input: 5000,
        assertion: function () {
          cy.aliasTableRows("Mina Accounts", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 25);
          cy.assertForEachColumnValue("Mina Accounts", "Balance", (text) => {
            let balance = parseFormattedNumber(text);
            expect(balance).to.be.lte(5000);
          });
        },
      },
      {
        column: "Balance",
        input: "5000.1234",
        assertion: function () {
          cy.aliasTableRows("Mina Accounts", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 25);
          cy.assertForEachColumnValue("Mina Accounts", "Balance", (text) => {
            let balance = parseFormattedNumber(text);
            expect(balance).to.be.lte(5000.1234);
          });
        },
      },
      {
        column: "Public Key",
        input: "B62qpqCBExtxzfHUPkmrrfmYhXZyg3V7pSmwuxHMzTi8E6gBbopauJS",
        assertion: function () {
          cy.aliasTableRows("Mina Accounts", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 1);
          cy.assertForEachColumnValue("Mina Accounts", "Public Key", (text) => {
            expect(text).to.equal(ROMEK_ADDRESS);
          });
          cy.tableColumnValuesEqual(
            "Mina Accounts",
            "Username",
            ROMEK_USERNAME,
          );
        },
      },
      {
        column: "Username",
        input: "Romek",
        assertion: function () {
          cy.aliasTableRows("Mina Accounts", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 1);
          cy.assertForEachColumnValue("Mina Accounts", "Username", (text) => {
            expect(text).to.equal(ROMEK_USERNAME);
          });
          cy.tableColumnValuesEqual(
            "Mina Accounts",
            "Username",
            ROMEK_USERNAME,
          );
        },
      },
      {
        column: "Delegate",
        input: "B62qpqCBExtxzfHUPkmrrfmYhXZyg3V7pSmwuxHMzTi8E6gBbopauJS",
        assertion: function () {
          cy.aliasTableRows("Mina Accounts", "table-rows");
          cy.assertForEachColumnValue("Mina Accounts", "Delegate", (text) => {
            expect(text).to.equal(ROMEK_ADDRESS);
          });
        },
      },
    ],
  },
  tests: [
    () => {
      cy.assertStandardRowLimits("Mina Accounts");
    },
    () => {
      cy.intercept("POST", "/graphql").as("graphql");
      cy.visit("/addresses/accounts");
      cy.wait("@graphql").then(() => {
        cy.wait(1000);
        cy.assertLoadNextWorks("Mina Accounts", "Balance");
      });
    },
  ],
};
