// Test suite data for: /commands/user
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
  url: "/commands/user",
  table: {
    heading: "User Commands",
    columns: [
      "Height",
      "Txn Hash",
      "Date",
      "Type",
      "Status",
      "From",
      "To",
      "Nonce",
      "Fee",
      "Amount",
    ],
    filter_tests: [
      {
        column: "Height",
        input: 2000,
        assertion: function () {
          cy.assertForEachColumnValue("User Commands", "Height", (text) => {
            let height = parseFormattedNumber(text);
            expect(height).to.be.lte(2000);
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
        column: "From",
        input: "B62qre3erTHfzQckNuibViWQGyyKwZseztqrjPZBv6SQF384Rg6ESAy",
        assertion: function () {
          cy.assertForEachColumnValue("User Commands", "From", (text) => {
            expect(text).to.equal(FIRST_SENDER_ADDRESS);
          });
        },
      },
      {
        column: "To",
        input: "B62qjYanmV7y9njVeH5UHkz3GYBm7xKir1rAnoY4KsEYUGLMiU45FSM",
        assertion: function () {
          cy.assertForEachColumnValue("User Commands", "To", (text) => {
            expect(text).to.equal(FIRST_RECIPIENT_ADDRESS);
          });
        },
      },
    ],
  },
  tests: [
    () => {
      cy.assertStandardRowLimits("User Commands");
    },
    () => {
      cy.get("select#txn-status").as("txn-applied");
      cy.get("select#canonical-selection").as("canonical");
      ["Failed", "Applied"].forEach((txnApplied) => {
        ["Non-Canonical", "Canonical"].forEach((canonical) => {
          cy.get("@txn-applied")
            .select(txnApplied)
            .should("have.value", txnApplied);
          cy.get("@canonical")
            .select(canonical)
            .should("have.value", canonical);
          cy.intercept("POST", "/graphql").as("graphql");
          cy.wait("@graphql").then(() => {
            cy.assertForEachColumnValue("User Commands", "Status", (text) => {
              expect(text).to.be.eq(txnApplied);
            });
            cy.clickLinkInTable(0, "Txn Hash", "User Commands");
            cy.testSpotlightValue("Status", txnApplied);
            cy.testSpotlightValue(
              "Canonical",
              "" + (canonical === "Canonical"),
            );
            cy.go("back");
          });
        });
      });
    },
    () => {
      cy.intercept("POST", "/graphql").as("graphql");
      cy.visit("/commands/user?q-height=25");
      cy.wait("@graphql").then(() => {
        cy.wait(1000);
        cy.assertLoadNextWorks("User Commands", "Height", {
          button_text: "Load Next",
          expected_button_state: "be.disabled",
        });
      });
    },
  ],
};