// Test suite data for: /commands/internal
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
  url: "/commands/internal",
  table: {
    heading: "Internal Commands",
    columns: ["Height", "State Hash", "Recipient", "Fee", "Type", "Date"],
    filter_tests: [
      {
        column: "Height",
        input: 2000,
        assertion: function () {
          cy.assertForEachColumnValue("Internal Commands", "Height", (text) => {
            let height = parseFormattedNumber(text);
            expect(height).to.be.lte(2000);
          });
        },
      },
      {
        column: "State Hash",
        input: "3NLgCqncc6Ct4dcuhaG3ANQbfWwQCxMXu4MJjwGgRKxs6p8vQsZf",
        assertion: function () {
          cy.aliasTableRows("Internal Commands", "table-rows");
          cy.get("@table-rows").should("have.length.greaterThan", 1);
          cy.assertForEachColumnValue(
            "Internal Commands",
            "State Hash",
            (text) => {
              expect(text).to.equal(ROMEK_BLOCK_STATE_HASH);
            },
          );
        },
      },
      {
        column: "Recipient",
        input: "B62qnucUMHz7Dw2ReNgWhmR5XCvPeQjJWPReuQ8GwPyY4qj1otGBiKr",
        assertion: function () {
          cy.assertForEachColumnValue(
            "Internal Commands",
            "Recipient",
            (text) => {
              expect(text).to.equal(
                "B62qnucUMHz7Dw2ReNgWhmR5XCvPeQjJWPReuQ8GwPyY4qj1otGBiKr",
              );
            },
          );
        },
      },
    ],
  },
  tests: [
    () => {
      cy.assertStandardRowLimits("Internal Commands");
    },
    () => {
      cy.intercept("POST", "/graphql").as("graphql");
      cy.visit("/commands/internal?q-height=25");
      cy.wait("@graphql").then(() => {
        cy.wait(1000);
        cy.assertLoadNextWorks("Internal Commands", "Height", {
          button_text: "Load Next",
          expected_button_state: "be.disabled",
        });
      });
    },
  ],
};
