// Test suite data for: /blocks
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
  url: "/blocks",
  table: {
    heading: "Blocks",
    columns: [
      "Height",
      "State Hash",
      "Slot",
      "Date",
      "Block Producer",
      "Coinbase",
      "User Commands",
      "SNARKs",
      "Coinbase Receiver",
    ],
    filter_tests: [
      {
        column: "Height",
        input: 2000,
        assertion: function () {
          cy.assertForEachColumnValue("Blocks", "Height", (text) => {
            let height = parseFormattedNumber(text);
            expect(height).to.be.lte(2000);
          });
        },
      },
      {
        column: "State Hash",
        input: "3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ",
        assertion: function () {
          cy.aliasTableRows("Blocks", "table-rows");
          cy.get("@table-rows").should("have.lengthOf", 1);
          cy.assertForEachColumnValue("Blocks", "State Hash", (text) => {
            expect(text).to.equal(GENESIS_BLOCK_BLOCK_HASH);
          });
        },
      },
      {
        column: "Slot",
        input: 90000,
        assertion: function () {
          cy.assertForEachColumnValue("Blocks", "Slot", (text) => {
            let height = parseFormattedNumber(text);
            expect(height).to.be.lte(90000);
            expect(height).to.be.gt(SLOTS_PER_EPOCH);
          });
        },
      },
      {
        column: "Block Producer",
        input: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
        assertion: function () {
          cy.assertForEachColumnValue("Blocks", "Block Producer", (text) => {
            expect(text).to.equal(FIRST_BLOCK_PRODUCER_ADDRESS);
          });
        },
      },
    ],
  },
  tests: [
    () => {
      cy.assertStandardRowLimits("Blocks");
    },
    () => {
      cy.get("select#canonical-selection").as("canonical");
      cy.get("@canonical").select("Canonical");
      cy.intercept("POST", "/graphql").as("graphql");
      cy.wait("@graphql").then(() => {
        cy.aliasTableRows("Blocks", "table-rows");
        cy.get("@table-rows").find(".non-canonical").should("not.exist");
        cy.get("@table-rows").find(".canonical").should("exist");
      });

      cy.get("@canonical").select("Non-Canonical");
      cy.intercept("POST", "/graphql").as("graphql");
      cy.wait("@graphql").then(() => {
        cy.aliasTableRows("Blocks", "table-rows");
        cy.get("@table-rows").find(".non-canonical").should("exist");
        cy.get("@table-rows").find(".canonical").should("not.exist");
      });

      cy.get("@canonical").select("All");
      cy.intercept("POST", "/graphql").as("graphql");
      cy.wait("@graphql").then(() => {
        cy.aliasTableRows("Blocks", "table-rows");
        cy.get("@table-rows").find(".non-canonical").should("exist");
        cy.get("@table-rows").find(".canonical").should("exist");
      });
    },
    () => {
      cy.intercept("POST", "/graphql").as("graphql");
      cy.visit("/blocks?q-height=25");
      cy.wait("@graphql").then(() => {
        cy.wait(1000);
        cy.assertLoadNextWorks("Blocks", "Height", {
          button_text: "Load Next",
          expected_button_state: "be.disabled",
        });
      });
    },
  ],
};
