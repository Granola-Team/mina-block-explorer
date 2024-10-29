// Test suite data for: /addresses/accounts
const { parseFormattedNumber } = require("../../helpers");
const {
  FIRST_BLOCK_PRODUCER_ADDRESS,
  FIRST_RECIPIENT_ADDRESS,
  FIRST_SENDER_ADDRESS,
  GENESIS_BLOCK_BLOCK_HASH,
  BLOCK_STATE_HASH_MIXED_USER_COMMANDS,
  ROMEK_ADDRESS,
  ROMEK_MINA_NAMING_SERVICE_TXN_HASH,
  ROMEK_USERNAME,
  SLOTS_PER_EPOCH,
  MINA_NAMING_SERVICE_ADDRESS,
  ROMEK_BLOCK_STATE_HASH,
  VETAL_BLOCK_STATE_HASH,
  ROMEK_NAMING_MEMO,
  SNZ_USERNAME,
  SNZPOOL_ADDRESS,
} = require("../../constants");

module.exports = {
  tag: "@tier2",
  url: "/addresses/accounts",
  table: {
    heading: "Accounts",
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
          cy.aliasTableRows("Accounts", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 25);
          cy.assertForEachColumnValue("Accounts", "Balance", (text) => {
            let balance = parseFormattedNumber(text);
            expect(balance).to.be.lte(5000);
          });
        },
      },
      {
        column: "Public Key",
        input: "B62qpqCBExtxzfHUPkmrrfmYhXZyg3V7pSmwuxHMzTi8E6gBbopauJS",
        assertion: function () {
          cy.aliasTableRows("Accounts", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 1);
          cy.assertForEachColumnValue("Accounts", "Public Key", (text) => {
            expect(text).to.equal(ROMEK_ADDRESS);
          });
          cy.tableColumnValuesEqual("Accounts", "Username", ROMEK_USERNAME);
        },
      },
      {
        column: "Username",
        input: "Romek",
        assertion: function () {
          cy.aliasTableRows("Accounts", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 1);
          cy.assertForEachColumnValue("Accounts", "Username", (text) => {
            expect(text).to.equal(ROMEK_USERNAME);
          });
          cy.tableColumnValuesEqual("Accounts", "Username", ROMEK_USERNAME);
        },
      },
      {
        column: "Delegate",
        input: "B62qpqCBExtxzfHUPkmrrfmYhXZyg3V7pSmwuxHMzTi8E6gBbopauJS",
        assertion: function () {
          cy.aliasTableRows("Accounts", "table-rows");
          cy.assertForEachColumnValue("Accounts", "Delegate", (text) => {
            expect(text).to.equal(ROMEK_ADDRESS);
          });
        },
      },
    ],
  },
  tests: [
    () => {
      cy.assertStandardRowLimits("Accounts");
    },
  ],
};
