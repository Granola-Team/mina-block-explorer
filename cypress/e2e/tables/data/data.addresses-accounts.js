import { parseFormattedNumber } from "../../helpers.js";
import { ROMEK_ADDRESS, ROMEK_USERNAME } from "../../constants.js";

export const url = "/addresses/accounts";
export const table = {
  heading: "MINA Accounts",
  columns: [
    "Type",
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
      column: "Type",
      input: "Zkapp",
      filter_type: "select",
      assertion: function () {
        cy.assertTableMetadataCorrect("MINA Accounts", 27, 1);
        cy.assertForEachColumnValue("MINA Accounts", "Type", (text) => {
          expect(text).to.be.eq("Zkapp");
        });
      },
    },
    {
      column: "Public Key",
      input: "B62qpqCBExtxzfHUPkmrrfmYhXZyg3V7pSmwuxHMzTi8E6gBbopauJS",
      assertion: function () {
        cy.aliasTableRows("MINA Accounts", "table-rows");
        cy.get("@table-rows").should("have.lengthOf", 1);
        cy.assertForEachColumnValue("MINA Accounts", "Public Key", (text) => {
          expect(text).to.equal(ROMEK_ADDRESS);
        });
        cy.tableColumnValuesEqual("MINA Accounts", "Username", ROMEK_USERNAME);
      },
    },
    {
      column: "Balance",
      input: "5000.1234",
      assertion: function () {
        cy.aliasTableRows("MINA Accounts", "table-rows");
        cy.get("@table-rows").should("have.lengthOf", 25);
        cy.assertForEachColumnValue("MINA Accounts", "Balance", (text) => {
          let balance = parseFormattedNumber(text);
          expect(balance).to.be.lte(5000.1234);
        });
      },
    },
    {
      column: "Delegate",
      input: "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9",
      assertion: function () {
        cy.aliasTableRows("MINA Accounts", "table-rows");
        cy.assertForEachColumnValue("MINA Accounts", "Delegate", (text) => {
          expect(text).to.equal(
            "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9",
          );
        });
      },
    },
  ],
};
export const tests = [
  () => {
    cy.assertStandardRowLimits("MINA Accounts");
  },
  // TODO: uncomment when https://github.com/Granola-Team/mina-indexer/issues/1869 is closed
  // and the indexer is updated locally
  // () => {
  //   cy.visit("/addresses/accounts");
  //   cy.waitUntilTableLoads("MINA Accounts");
  //   cy.assertLoadNextWorks("MINA Accounts", "Balance");
  // },
];
export default {
  url,
  table,
  tests,
};
