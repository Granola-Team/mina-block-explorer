// Test suite data for: /snarks
const { parseFormattedNumber } = require("../../helpers");
const { VETAL_BLOCK_STATE_HASH } = require("../../constants");

module.exports = {
  tag: "@tier2",
  url: "/snarks",
  table: {
    heading: "SNARKs",
    columns: ["Height", "State Hash", "Date", "Prover", "Fee"],
    filter_tests: [
      {
        column: "Height",
        input: 2000,
        assertion: function () {
          cy.assertForEachColumnValue("SNARKs", "Height", (text) => {
            let height = parseFormattedNumber(text);
            expect(height).to.be.lte(2000);
          });
        },
      },
      {
        column: "State Hash",
        input: "3NKrxKGr3JpYT2CzAFUeUb89ae6MFMsVWFX1QLYqYNJp1ffHR4ej",
        assertion: function () {
          cy.aliasTableRows("SNARKs", "table-rows");
          cy.get("@table-rows").should("have.length.greaterThan", 1);
          cy.assertForEachColumnValue("SNARKs", "State Hash", (text) => {
            expect(text).to.equal(VETAL_BLOCK_STATE_HASH);
          });
        },
      },
      {
        column: "Prover",
        input: "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC",
        assertion: function () {
          cy.assertForEachColumnValue("SNARKs", "Prover", (text) => {
            expect(text).to.equal(
              "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC",
            );
          });
        },
      },
    ],
  },
  tests: [
    () => {
      cy.assertStandardRowLimits("SNARKs");
    },
    () => {
      cy.intercept("POST", "/graphql").as("graphql");
      cy.visit("/snarks?row-limit=100&q-height=149");
      cy.wait("@graphql").then(() => {
        cy.wait(1000);
        cy.assertLoadNextWorks("SNARKs", "Height", {
          button_text: "Load Next",
          expected_button_state: "be.disabled",
        });
      });
    },
  ],
};
