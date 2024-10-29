// Test suite data for: /addresses/accounts/B62qpqCBExtxzfHUPkmrrfmYhXZyg3V7pSmwuxHMzTi8E6gBbopauJS
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
  url: "/addresses/accounts/B62qpqCBExtxzfHUPkmrrfmYhXZyg3V7pSmwuxHMzTi8E6gBbopauJS",
  table: {
    heading: "User Commands",
    columns: [
      "Height",
      "Txn Hash",
      "Nonce",
      "Date",
      "Type",
      "Direction",
      "Counterparty",
      "Amount/Fee",
    ],
    filter_tests: [
      {
        column: "Height",
        input: 2500,
        assertion: function () {
          cy.assertForEachColumnValue("User Commands", "Height", (text) => {
            let height = parseFormattedNumber(text);
            expect(height).to.be.lte(2500);
          });
        },
      },
      {
        column: "Txn Hash",
        input: "CkpYyMV4jDtgKfbz6hCUVB6J8jYfJd85A7mvtVw7ydKLuoCK5GS25",
        assertion: function () {
          cy.aliasTableRows("User Commands", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 1);
          cy.assertForEachColumnValue("User Commands", "Txn Hash", (text) => {
            expect(text).to.contain(ROMEK_MINA_NAMING_SERVICE_TXN_HASH);
            expect(text).to.contain(ROMEK_NAMING_MEMO);
          });
        },
      },
      {
        column: "Counterparty",
        input: "B62qjzJvc59DdG9ahht9rwxkEz7GedKuUMsnaVTuXFUeANKqfBeWpRE",
        assertion: function () {
          cy.assertForEachColumnValue(
            "User Commands",
            "Counterparty",
            (text) => {
              expect(text).to.equal(MINA_NAMING_SERVICE_ADDRESS);
            },
          );
        },
      },
    ],
  },
  tests: [
    () => {
      let expected_fields = ["Balance", "Delegate", "Nonce", "Updated Block #"];
      cy.testSpotlight("Account Spotlight", ROMEK_ADDRESS, expected_fields);
    },
    () => {
      cy.get("#spotlight-meta").should("contain", ROMEK_USERNAME);
      cy.testSpotlightValue("Balance", "Includes 1 MINA account creation fee");
    },
  ],
};
